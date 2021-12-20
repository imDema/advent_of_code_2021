use std::fmt::Debug;
use std::io::Read;

use nom::IResult;
use nom::multi::{many_till, count};
use num_bigint::BigUint;
use nom::bits::bits;
use nom::bits::complete::{tag, take};

enum Packet {
    Literal(PacketLiteral),
    Operator(PacketOp),
}

impl Packet {
    pub fn from_bytes(bytes: &'_ [u8]) -> anyhow::Result<Self> {
        use nom::error::Error;
        let (bytes, packet) = bits::<_,_,Error<(&[u8], usize)>, Error<&[u8]>, _>(parse_packet)(bytes).unwrap();
        assert_eq!(bytes.len(), 0);
        Ok(packet)
    }

    pub fn version_sum(&self) -> usize {
        match self {
            Packet::Literal(p) => p.header.version as usize,
            Packet::Operator(p) => {
                let sub_sum: usize = p.sub_packets.iter()
                    .map(|s| s.version_sum())
                    .sum();
                sub_sum + p.header.version as usize
            }
        }
    }

    pub fn compute(&self) -> BigUint {
        match self {
            Packet::Literal(p) => p.value.clone(),
            Packet::Operator(p) => {
                let mut sub_vals = p.sub_packets.iter()
                    .map(|p| p.compute());

                let ret = match p.header.type_id {
                    0 => sub_vals.sum(),
                    1 => sub_vals.product(),
                    2 => sub_vals.min().unwrap(),
                    3 => sub_vals.max().unwrap(),
                    5 => if sub_vals.next().unwrap() > sub_vals.next().unwrap() { 1usize } else { 0usize }.into(),
                    6 => if sub_vals.next().unwrap() < sub_vals.next().unwrap() { 1usize } else { 0usize }.into(),
                    7 => if sub_vals.next().unwrap() == sub_vals.next().unwrap() { 1usize } else { 0usize }.into(),
                    _ => unreachable!("Invalid type_id"),
                };
                ret
            }
        }
    }
}

impl Debug for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(p) => f.write_fmt(format_args!("{}", p.value))?,
            Self::Operator(p) => {
                f.write_str("(")?;
                f.write_str(match p.header.type_id {
                    0 => "sum",
                    1 => "prod",
                    2 => "min",
                    3 => "max",
                    5 => "gt",
                    6 => "lt",
                    7 => "eq",
                    _ => unreachable!("Invalid type_id"),
                })?;
                for s in p.sub_packets.iter() {
                    f.write_str(" ")?;
                    s.fmt(f)?;
                }
                f.write_str(")")?;
            }
        }
        Ok(())
    }
}

struct PacketHeader {
    version: u8,
    type_id: u8,
}

struct PacketLiteral {
    header: PacketHeader,
    value: BigUint,
}

struct PacketOp {
    header: PacketHeader,
    sub_packets: Vec<Packet>,
}

fn parse_packet(i: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
    let (i, header) = parse_header(i)?;

    match header.type_id {
        4 => parse_literal(i, header),
        _ => parse_operator(i, header),
    }
}

fn parse_header(i: (&[u8], usize)) -> IResult<(&[u8], usize), PacketHeader> {
    let (i, version) = take(3usize)(i)?;
    let (i, type_id) = take(3usize)(i)?;

    Ok((i, PacketHeader{ version, type_id, }))
}

fn parse_literal(i: (&[u8], usize), header: PacketHeader) -> IResult<(&[u8], usize), Packet> {
    let chunk = |i| {
        let (i, _) = tag(0b1, 1usize)(i)?;
        take(4usize)(i)
    };
    let last_chunk = |i| {
        let (i, _) = tag(0b0, 1usize)(i)?;
        take(4usize)(i)
    };
    let (i, (mut list, last)) = many_till(chunk, last_chunk)(i)?;
    list.push(last);
    let value: BigUint= list.into_iter()
        .fold(BigUint::new(vec![0]), |val, chk: u8| {
            (val << 4) + chk
        });

    Ok((i, Packet::Literal(PacketLiteral {
        header,
        value,
    })))
}

fn parse_operator(i: (&[u8], usize), header: PacketHeader) -> IResult<(&[u8], usize), Packet> {
    let (i, mode) = take(1usize)(i)?;
    let (i, sub_packets) = match mode {
        0 => {
            let (i0, bit_len) = take(15usize)(i)?;
            let mut sub_packets = Vec::new();
            let mut i = i0.clone();
            while delta_bit(i0, i) < bit_len {
                let (new_i, new_p) = parse_packet(i)?;
                sub_packets.push(new_p);
                i = new_i;
            }
            (i, sub_packets)
        }
        1 => {
            let (i, num_sub) = take(11usize)(i)?;
            count(parse_packet, num_sub)(i)?
        }
        _ => unreachable!(),
    };

    Ok((i, Packet::Operator(PacketOp {
        header,
        sub_packets,
    })))
}

/// Count difference in bit length in nom format.
fn delta_bit(a: (&[u8], usize), b: (&[u8], usize)) -> usize {
    let ra = a.0.len() * 8 - a.1;
    let rb = b.0.len() * 8 - b.1;
    ra - rb
}

fn main() -> anyhow::Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;

    let bytes = hex::decode(buf.trim())?;

    let packet = Packet::from_bytes(&bytes[..])?;

    let v_sum = packet.version_sum();
    let val = packet.compute();

    eprintln!("{:?}", &packet);

    println!("version sum: {}\nvalue: {}", v_sum, val);

    Ok(())
}

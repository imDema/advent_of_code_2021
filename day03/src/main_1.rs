use std::io::BufRead;

fn parse_bits(s: &str) -> Vec<bool> {
    s.chars()
        .map(|c| match c {
            '1' => true,
            '0' => false,
            _ => panic!(),
        })
        .collect()
}


fn main() {
    let stdin = std::io::stdin();

    let bits: Vec<_> = stdin.lock().lines()
        .map(|l| parse_bits(&l.unwrap()))
        .collect();

    let len = bits.len();

    let counts = bits.into_iter()
        .fold(Vec::new(), |mut acc, x| {
            while acc.len() < x.len() {
                acc.push(0usize);
            }
            acc.iter_mut()
                .zip(x.into_iter())
                .for_each(|(a, b)| if b { *a += 1 });
            acc
        });

    dbg!(&counts);

    let (low, high) = counts.into_iter()
        .fold((0, 0), |(low, high), x| {
            if x >= (len + 1) / 2 {
                (low << 1, (high << 1) | 1)
            } else {
                ((low << 1) | 1, high << 1)
            }
        });

    println!("gamma: {}, eps: {}, res: {}", low, high, low*high);
}

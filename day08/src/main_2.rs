use std::{io::BufRead, str::FromStr, collections::{HashSet, HashMap}};

use anyhow::Context;
use itertools::Itertools;

const S: usize = 7;

struct Digit {
    segments: Vec<HashSet<u8>>,
}

#[derive(Clone, Copy)]
enum FilterOp {
    Keep,
    Drop,
}

impl Digit {
    pub fn new() -> Self {
        let segments = (0..S).map(|_| (0..S as u8).collect()).collect();
        Self {
            segments
        }
    }

    fn filter(&mut self, seg: u8, values: &[u8], op: FilterOp) {
        let s = &mut self.segments[seg as usize];
        if s.len() == 1 {
            return;
        }
        match op {
            FilterOp::Keep => s.retain(|e| values.contains(e)),
            FilterOp::Drop => s.retain(|e| !values.contains(e)),
        }
        let l = s.len();
        if l == 1 {
            let sig = *s.iter().nth(0).unwrap();
            for i in 0..S as u8{
                self.filter(i, &[sig], FilterOp::Drop);
            }
        }
    }

    pub fn process_signals(&mut self, sigs: &[Vec<u8>]) {
        for s in sigs {
            let segs = match s.len() {
                2 => vec![2, 5],
                3 => vec![0, 2, 5],
                4 => vec![1, 2, 3, 5],
                5 => vec![0, 3, 6],
                6 => vec![0, 1, 5, 6],
                _ => continue,
            };
            for seg in segs {
                self.filter(seg, &s, FilterOp::Keep);
            }
        }

        for i in 0..S {
            println!("{}: {:?}", i, self.segments[i]);
        }
    }

    pub fn get_encoding(&self) -> Encoding {
        let map = self.segments.iter().enumerate()
            .map(|(i, v)| {
                assert_eq!(v.len(), 1);
                (*v.into_iter().nth(0).unwrap(), i as u8)
            })
            .collect();
        Encoding{ map }
    }
}

struct Encoding {
    pub map: HashMap<u8, u8>
}

impl Encoding {
    pub fn decode(&self, signals: &[u8]) -> u8 {
        let mut segments: Vec<u8> = signals.into_iter()
            .map(|s| self.map[s])
            .collect();

        segments.sort_unstable();
        match &segments[..] { // This is suffering
            &[0, 1, 2, 4, 5, 6] => 0,
            &[2, 5] => 1,
            &[0, 2, 3, 4, 6] => 2,
            &[0, 2, 3, 5, 6] => 3,
            &[1, 2, 3, 5] => 4,
            &[0, 1, 3, 5, 6] => 5,
            &[0, 1, 3, 4, 5, 6] => 6,
            &[0, 2, 5] => 7,
            &[0, 1, 2, 3, 4, 5, 6] => 8,
            &[0, 1, 2, 3, 5, 6] => 9,
            _ => panic!(),
        }
    }
}

struct InputLine {
    sigs: Vec<Vec<u8>>,
    digits: Vec<Vec<u8>>,
}

impl FromStr for InputLine{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (sigs, digits) = s.split('|')
            .collect_tuple()
            .context("Wrong input format!")?;

        let parse_digit = |c| (c as u8 - 'a' as u8);
        let sigs = sigs.split_whitespace()
            .map(|w| w.chars().map(parse_digit).collect())
            .collect();

        let digits = digits.split_whitespace()
            .map(|w| w.chars().map(parse_digit).collect())
            .collect();

        Ok(Self{sigs, digits})
    }
}

fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin();
    let inputs: Vec<InputLine> = stdin.lock().lines()
        .map(|l| l.unwrap().parse())
        .collect::<Result<_,_>>()?;

    let mut sum = 0;
    for line in inputs {
        let mut d = Digit::new();
        d.process_signals(&line.sigs);
        let enc = d.get_encoding();
        let n = line.digits.into_iter()
            .map(|d| enc.decode(&d[..]))
            .fold(0, |acc, x| {
                acc * 10 + x as usize
            });
        sum += n;
    }

    println!("sum: {}", sum);

    Ok(())
}

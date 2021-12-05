use std::{collections::HashMap, str::FromStr, io::BufRead};
use anyhow::Context;
use itertools::Itertools;


type Coord = (i32, i32);

fn parse_coord(s: &str) -> anyhow::Result<Coord> {
    let (x, y) = s.split(',')
        .map(|n| i32::from_str_radix(n, 10).unwrap())
        .collect_tuple()
        .context("Wrong input format, missing `,` in coord")?;
    Ok((x, y))
}

#[derive(Debug)]
struct Line {
    pub from: Coord,
    pub to: Coord,
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s.split(" -> ")
            .collect_tuple()
            .context("Wrong input format, missing ` -> `")?;
        Ok(Self{
            from: parse_coord(from)?,
            to: parse_coord(to)?,
        })
    }
}

struct Floor {
    map: HashMap<Coord, u32>,
}

impl Floor {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub fn line(&mut self, line: Line) {
        let Line {from, to} = line;
        let points: Vec<_> = if from.0 != to.0 && from.1 == to.1 {
            let x0 = from.0.min(to.0);
            let x1 = from.0.max(to.0);
            (x0..=x1).map(|x| (x, from.1)).collect()
        } else if from.1 != to.1 && from.0 == to.0 {
            let y0 = from.1.min(to.1);
            let y1 = from.1.max(to.1);
            (y0..=y1).map(|y| (from.0, y)).collect()
        } else {
            return;
        };

        for p in points {
            *self.map.entry(p).or_default() += 1;
        }
    }

    pub fn count_dangerous(&self) -> usize {
        self.map.values()
            .filter(|&&v| v > 1)
            .count()
    }
}

fn main() {
    let stdin = std::io::stdin();
    let floor = stdin.lock().lines()
        .map(|l| l.unwrap().parse().unwrap())
        .fold(Floor::new(), |mut floor, x| {
            floor.line(x);
            floor
        });
        
    let danger = floor.count_dangerous();

    println!("dangerous: {}", danger);
}

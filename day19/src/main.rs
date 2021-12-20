use std::str::FromStr;

use anyhow::Context;
use itertools::Itertools;
use rand::{Rng, thread_rng};
use rayon::prelude::*;
use nalgebra as na;

const THRESH: i64 = 1000;

type Coord = na::Vector3<i64>;

fn dist(a: &Coord, b: &Coord) -> i64 {
    let d = a - b;
    d[0]*d[0] + d[1]*d[1] + d[2]*d[2]
}

fn parse_coord(s: &str) -> anyhow::Result<Coord> {
    Ok(Coord::from_iterator(s.split(',')
        .map(|w| w.parse())
        .collect::<Result<Vec<_>,_>>()?
        .into_iter()))
}

struct Scanner {
    id: usize,
    position: Coord,
    rotation: usize,
    beacons: Vec<Coord>,
    fixed: bool,
}

impl Scanner {
    pub fn rotate(&mut self, rotation: usize) {
        assert!(rotation < 24);
        self.rotation = rotation;
    }

    pub fn beacons(&self) -> impl Iterator<Item=Coord> + '_ {
        self.beacons.iter().map(|c| self.translate_relative(*c))
    }

    pub fn scan_match(&mut self, other: &Scanner) {
        let pairs = self.beacons().par_bridge()
            .map(|b| {
                let (dist, closest) = other.beacons()
                    .map(|o| (dist(&b, &o), o))
                    .fold1(|acc, x| if x.0 < acc.0 { x } else { acc })
                    .unwrap();
                (dist, b, closest)
            }).filter(|t| t.0 < )
    }

    fn translate_relative(&self, coord: Coord) -> Coord {
        let mut x = coord;
        
        let mut r = self.rotation;
        x.swap_rows(0, r % 3); r /= 3;
        x.swap_rows(1, 1 + r % 2); r /= 2;
        (r % 2 > 0).then(|| x[0] *= -1); r /= 2;
        (r % 2 > 0).then(|| x[1] *= -1); r /= 2;
        (r % 2 > 0).then(|| x[2] *= -1); r /= 2;

        assert_eq!(r, 0);

        x + self.position
    }
}

impl FromStr for Scanner {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let id = lines.next().context("Empty input")?
            .split_whitespace()
            .nth(2).context("Wrong header format")?
            .parse()?;

        let beacons = lines.map(parse_coord)
            .collect::<Result<Vec<_>,_>>()?;

        if id == 0 {
            Ok(Self {
                id,
                beacons,
                position: Coord::zeros(),
                rotation: 0,
                fixed: true,
            })
        } else {
            Ok(Self {
                id,
                beacons,
                position: Coord::from_iterator([
                    thread_rng().gen_range(-1000..1000),
                    thread_rng().gen_range(-1000..1000),
                    thread_rng().gen_range(-1000..1000),
                ]),
                rotation: thread_rng().gen_range(0..4),
                fixed: true,
            })
        }
    }
}

fn main() {
    println!("Hello, world!");
}

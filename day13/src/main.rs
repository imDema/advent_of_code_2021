use std::{collections::HashSet, str::FromStr, io::Read};

use anyhow::Context;
use itertools::Itertools;
use regex::Regex;
use plotters::prelude::*;

lazy_static::lazy_static!(
    static ref RE_FOLD: Regex = Regex::new(r"^fold along (x|y)=(\d+)$").unwrap();
);

#[derive(Debug, Clone, Copy)]
enum Fold {
    X(u32),
    Y(u32)
}

impl FromStr for Fold {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = RE_FOLD.captures(s).context("Invalid fold format")?;
        let n = caps[2].parse()?;
        Ok(match &caps[1] {
            "x" => Fold::X(n),
            "y" => Fold::Y(n),
            _ => unreachable!(),
        })
    }
}


type Coord = (u32, u32);

struct Map {
    set: HashSet<Coord>,
}

impl Map {
    pub fn fold(&mut self, fold: Fold) {
        self.set = self.set.iter()
            .filter_map(|c| {
                match fold {
                    Fold::X(x) if c.0 > x => Some((x - (c.0 - x), c.1)),
                    Fold::X(x) if c.0 < x => Some(*c),
                    Fold::X(x) if c.0 == x => None,
                    Fold::Y(y) if c.1 > y => Some((c.0, y - (c.1 - y))),
                    Fold::Y(y) if c.1 < y => Some(*c),
                    Fold::Y(y) if c.1 == y => None,
                    _ => unreachable!(),
                }
            }).collect();
    }

    pub fn count(&self) -> usize {
        self.set.len()
    }

    pub fn plot(&self) -> anyhow::Result<()> {
        let root = BitMapBackend::new("out.png", (1024, 768)).into_drawing_area();

        let data: Vec<_> = self.set.iter().map(|x| *x).collect();
        let max = data.iter()
            .map(|c| c.0.max(c.1))
            .max()
            .context("No values!")?;

        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .margin(5)
            .build_cartesian_2d(0..max + 1, 0..max + 1)?;

        chart.configure_mesh()
            .draw()?;

        chart.draw_series(
            self.set.iter()
                .map(|(x, y)| Circle::new((*x, max - *y), 8, RED.filled())),
        )?;

        for j in 0..max + 1 {
            for i in 0..max + 1 {
                if self.set.contains(&(i, j)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("")
        }

        Ok(())
    }
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let set: HashSet<Coord> = s.lines()
            .map(|l| l.split(',')
                .map(|w| w.parse().unwrap())
                .collect_tuple()
                .context("Wrong line format"))
            .collect::<Result<_,_>>()?;

        Ok(Self {
            set
        })
    }
}

fn main() -> anyhow::Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;
    let mut sections = buf.split("\n\n");
    let mut map: Map = sections.next()
        .context("Wrong input sections")
        .and_then(|s| s.parse())?;
    let folds: Vec<Fold> = sections.next()
        .context("Wrong input sections")
        .and_then(|s| s.lines().map(|l| l.parse()).collect())?;

    map.fold(folds[0]);
    println!("First fold: {}", map.count());
    for fold in folds.into_iter().skip(1) {
        map.fold(fold);
    }
    map.plot()?;
    Ok(())
}

use std::{ops::{RangeInclusive}, str::FromStr, io::Read, sync::atomic::{AtomicUsize, Ordering}};

use anyhow::Context;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use regex::Regex;

lazy_static::lazy_static!(
    static ref RE_INPUT: Regex = Regex::new(r"^target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();
);

struct Target {
    pub x: RangeInclusive<i32>,
    pub y: RangeInclusive<i32>,
}

impl Target{
    pub fn contains(&self, (x, y): (i32, i32)) -> bool {
        self.x.contains(&x) && self.y.contains(&y)
    }
}

impl FromStr for Target {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cap = RE_INPUT.captures(s).context("Wrong input format")?;
        let (x0, x1) = (cap[1].parse()?, cap[2].parse()?);
        let (y0, y1) = (cap[3].parse()?, cap[4].parse()?);
        Ok(Self {
            x: x0..=x1,
            y: y0..=y1,
        })
    }
}

struct Probe {
    pose: (i32, i32),
    twist: (i32, i32),
}

impl Probe {
    pub fn new(twist: (i32, i32)) -> Self { Self { pose: (0, 0), twist } }

    pub fn step(&mut self) {
        self.pose.0 += self.twist.0;
        self.pose.1 += self.twist.1;
        self.twist.0 = self.twist.0 - self.twist.0.signum();
        self.twist.1 -=1;
    }

    pub fn hit(&self, target: &Target) -> bool {
        target.contains(self.pose)
    }

    pub fn missed(&self, target: &Target) -> bool {
        if self.pose.1 < *target.y.start() {
            return true;
        }
        if self.pose.0 > *target.x.end() && self.twist.0 >= 0 {
            return true;
        }
        if self.pose.0 < *target.x.start() && self.twist.0 <= 0 {
            return true;
        }
        let delta_x = sum_seq(self.twist.0);
        if self.pose.0 + delta_x < *target.x.start() {
            return true;
        }
        false
    }
}

fn sum_seq(n: i32) -> i32 {
    n * (n+1) / 2
}

fn main() -> anyhow::Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;
    let target: Target = buf.parse()?;

    let max_x = *target.x.end() + 1;
    let min_y = *target.y.start() - 1;
    let max_y = (*target.y.start() - 1).abs();
    let cnt = AtomicUsize::new(0);

    let max = (0..max_x).into_par_iter()
        .flat_map(|vx|
            (min_y..max_y).into_par_iter().map(move |vy| (vx, vy)))
        .filter_map(|twist| {
            let mut p = Probe::new(twist);
            while !p.missed(&target) {
                if p.hit(&target) {
                    let apex = sum_seq(twist.1);
                    cnt.fetch_add(1, Ordering::Relaxed);
                    return Some(apex)
                } else {
                    p.step();
                }
            };
            None
        }).max();

    println!("max: {:?}\ncount: {}", max, cnt.into_inner());

    Ok(())
}

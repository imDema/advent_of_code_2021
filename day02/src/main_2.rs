use anyhow::Context;
use itertools::Itertools;
use std::io::BufRead;
use std::str::FromStr;

enum Command {
    Fwd(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cmd, num) = s
            .split_whitespace()
            .collect_tuple()
            .context("Wrong command format")?;

        let x = i32::from_str(num)?;

        Ok(match cmd {
            "forward" => Self::Fwd(x),
            "up" => Self::Up(x),
            "down" => Self::Down(x),
            _ => unreachable!(),
        })
    }
}

#[derive(Default)]
struct Coord {
    x: i32,
    y: i32,
    aim: i32,
}

fn main() {
    let stdin = std::io::stdin();
    let pos = stdin.lock().lines()
        .map(|l| Command::from_str(&l.unwrap()).unwrap())
        .fold(Coord::default(), |Coord { x, y, aim }, cmd| match cmd {
            Command::Fwd(v) => Coord {
                x: x + v,
                y: y + v,
                aim,
            },
            Command::Down(v) => Coord { x, y, aim: aim + v },
            Command::Up(v) => Coord { x, y, aim: aim - v },
        });

    println!("x: {}, y: {}, res: {}", pos.x, pos.y, pos.x * pos.y);
}

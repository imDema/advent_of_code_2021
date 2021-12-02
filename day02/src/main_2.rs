use anyhow::Context;
use itertools::Itertools;
use std::str::FromStr;
use std::io::BufRead;

enum Command {
    Fwd(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cmd, num) = s.split_whitespace()
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
        .fold(Coord::default(), |mut c, cmd| {
            match cmd {
                Command::Fwd(v) => {
                    c.x += v;
                    c.y += v * c.aim;
                }
                Command::Down(v) => c.aim += v,
                Command::Up(v) => c.aim -= v,
            };
            c
        });

    println!("x: {}, y: {}, res: {}", pos.x, pos.y, pos.x * pos.y);

}

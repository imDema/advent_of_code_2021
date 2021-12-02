use anyhow::Context;
use itertools::Itertools;
use std::str::FromStr;
use std::io::BufRead;

enum Command {
    Fwd(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cmd, num) = s.split_whitespace()
            .collect_tuple()
            .context("Wrong command format")?;

        let x = u32::from_str(num)?;

        Ok(match cmd {
            "forward" => Self::Fwd(x),
            "up" => Self::Up(x),
            "down" => Self::Down(x),
            _ => unreachable!(),
        })
    }
}

fn main() {
    let stdin = std::io::stdin();
    let pos = stdin.lock().lines()
        .map(|l| Command::from_str(&l.unwrap()).unwrap())
        .fold((0, 0), |(x, y), cmd| {
            match cmd {
                Command::Fwd(v) => (x + v, y),
                Command::Down(v) => (x, y + v),
                Command::Up(v) => (x, y - v),
            }
        });

    println!("x: {}, y: {}, res: {}", pos.0, pos.1, pos.0 * pos.1);

}

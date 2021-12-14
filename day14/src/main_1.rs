use std::{collections::HashMap, str::FromStr, io::Read};

use anyhow::Context;
use itertools::Itertools;
use regex::Regex;

lazy_static::lazy_static!(
    static ref RE_RULE: Regex = Regex::new(r"^([A-Z]{2}) -> ([A-Z])$").unwrap();
);


struct Translator {
    rules: HashMap<[char; 2], char>
}

impl Translator {
    pub fn translate(&self, s: String) -> String {
        let mut out = String::with_capacity(s.len() * 2);
        for (cc, nc) in s.chars().zip(s.chars().skip(1)) {
            out.push(cc);
            if let Some(n) = self.rules.get(&[cc, nc]) {
                out.push(*n);
            }
        }
        out.push(s.chars().last().unwrap());
        out
    }
}

impl FromStr for Translator {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rules = s.lines()
            .map(|l| {
                let caps = RE_RULE.captures(l).context("Wrong rule format!")?;
                let lhs: [char; 2] = caps[1].chars().collect::<Vec<_>>().try_into().unwrap();
                let rhs = caps[2].chars().nth(0).unwrap();
                Ok((lhs, rhs))
            })
            .collect::<Result<_,Self::Err>>()?;

        Ok(Self {
            rules
        })
    }
}


fn main() -> anyhow::Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;

    let mut sections = buf.split("\n\n");

    let mut s = sections.nth(0)
        .context("Wrong input format!")?
        .to_owned();
    let tr: Translator = sections.nth(0)
        .context("Wrong input format!")?
        .parse()?;

    for _ in 0..10 {
        // eprintln!("{}", &s);
        s = tr.translate(s);
    }

    let (min, max) = s.chars()
        .counts()
        .into_iter()
        .map(|(_, v)| v)
        .minmax()
        .into_option()
        .unwrap();

    println!("min: {}, max: {}\nmax - min: {}", min, max, max - min);

    Ok(())
}

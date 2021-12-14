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
    pub fn update(&self, p: &mut Pairs) {
        let mut out = Pairs::default();
        for (pair, &count) in p.pairs() {
            let new = self.generate(pair, count);
            out.merge_full(new);
        }
        out.merge_atoms(&p);
        *p = out;
    }

    fn generate(&self, pair: &[char; 2], count: u64) -> Pairs {
        let mut atoms = HashMap::new();
        let mut pairs = HashMap::new();

        if let Some(c) = self.rules.get(pair) {
            atoms.insert(*c, count);
            pairs.insert([pair[0], *c], count);
            pairs.insert([*c, pair[1]], count);
        }

        Pairs::new(atoms, pairs)
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

#[derive(Default)]
struct Pairs {
    atoms: HashMap<char, u64>,
    pairs: HashMap<[char; 2], u64>,
}

impl Pairs {
    fn new(atoms: HashMap<char, u64>, pairs: HashMap<[char; 2], u64>) -> Self { Self { atoms, pairs } }

    pub fn pairs(&self) -> impl Iterator<Item=(&[char; 2], &u64)> {
        self.pairs.iter()
    }

    pub fn atoms(&self) -> impl Iterator<Item=(&char, &u64)> {
        self.atoms.iter()
    }

    pub fn merge_full(&mut self, other: Pairs) {
        self.merge_atoms(&other);
        for (p, c) in other.pairs {
            *self.pairs.entry(p).or_default() += c;
        }
    }

    pub fn merge_atoms(&mut self, other: &Pairs) {
        for (a, c) in other.atoms.iter() {
            *self.atoms.entry(*a).or_default() += c;
        }
    }
}

impl FromStr for Pairs {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut out = Pairs::default();
        for (cc, nc) in s.chars().zip(s.chars().skip(1)) {
            *out.atoms.entry(cc).or_default() += 1;
            *out.pairs.entry([cc, nc]).or_default() += 1;
        }
        *out.atoms.entry(s.chars().last().unwrap()).or_default() += 1;
        Ok(out)
    }
}

fn main() -> anyhow::Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;

    let mut sections = buf.split("\n\n");

    let mut p: Pairs = sections.nth(0)
        .context("Wrong input format!")?
        .parse()?;
    let tr: Translator = sections.nth(0)
        .context("Wrong input format!")?
        .parse()?;

    for _ in 0..40 {
        tr.update(&mut p);
    }

    let (min, max) = p
        .atoms()
        .into_iter()
        .map(|(_, v)| v)
        .minmax()
        .into_option()
        .unwrap();

    println!("min: {}, max: {}\nmax - min: {}", min, max, max - min);

    Ok(())
}

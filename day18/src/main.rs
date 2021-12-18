// THIS ONE IS BAD, VIEWER ADVISED


use std::{fmt::Debug, ops::Add, str::FromStr, io::Read};

use itertools::Itertools;
use rayon::prelude::*;

#[derive(Clone, Copy)]
enum Term {
    Num(u32),
    Open,
    Sep,
    Close,
}

#[derive(Clone)]
struct Num {
    terms: Vec<Term>
}

impl Num {
    pub fn reduce(&mut self) {
        loop {
            if self.explode() { continue; }
            if self.split() { continue; }
            break;
        }
    }

    fn explode(&mut self) -> bool {
        let mut lvl = 0;
        for i in 0..self.terms.len() {
            match self.terms[i] {
                Term::Open => lvl += 1,
                Term::Close => lvl -= 1,
                _ => {}
            }
            if lvl == 5 {
                let (left, right) = if let (Term::Num(l), Term::Num(r)) = (self.terms[i + 1], self.terms[i + 3]) {
                    (l, r)
                } else {
                    panic!();
                };
                for j in (0..i).rev() {
                    if let Term::Num(n) = &mut self.terms[j] {
                        *n += left;
                        break;
                    }
                }
                for j in i+4..self.terms.len() {
                    if let Term::Num(n) = &mut self.terms[j] {
                        *n += right;
                        break;
                    }
                }
                self.terms[i] = Term::Num(0);
                let new_len = self.terms.len()-4;
                for j in i+1..new_len {
                    self.terms[j] = self.terms[j + 4];
                }
                self.terms.truncate(new_len);
                return true;
            }
        }
        false
    }

    fn split(&mut self) -> bool {
        for i in 0..self.terms.len() {
            match self.terms[i] {
                Term::Num(n) if n >= 10 => {
                    let l = n / 2;
                    let r = n / 2 + n % 2;
                    self.terms.extend([Term::Num(0); 4]);
                    for j in (i+5..self.terms.len()).rev() {
                        self.terms[j] = self.terms[j.checked_sub(4).unwrap_or_else(|| panic!("\n{:?}\ni:{}\tj:{}", &self, i, j))];
                    }
                    self.terms[i] = Term::Open;
                    self.terms[i+1] = Term::Num(l);
                    self.terms[i+2] = Term::Sep;
                    self.terms[i+3] = Term::Num(r);
                    self.terms[i+4] = Term::Close;
                    return true;
                }
                _ => {}
            }
        }
        false
    }

    pub fn magnitude(&self) -> u64 {
        let mut mag = 0;
        let mut mult = 1;
        for &t in self.terms.iter() {
            match t {
                Term::Num(n) => mag += n as u64 * mult,
                Term::Open => mult *= 3,
                Term::Sep => mult = (mult / 3) * 2,
                Term::Close => mult /= 2,
            }
        }
        mag
    }
}

impl Add for Num {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut terms = Vec::with_capacity(self.terms.len() + rhs.terms.len() + 2);
        terms.push(Term::Open);
        terms.extend(self.terms);
        terms.push(Term::Sep);
        terms.extend(rhs.terms);
        terms.push(Term::Close);
        let mut new = Self { terms };
        new.reduce();
        new
    }
}

impl<'a> Add<&'a Num> for &'a Num {
    type Output = Num;

    fn add(self, rhs: Self) -> Self::Output {
        let mut terms = Vec::with_capacity(self.terms.len() + rhs.terms.len() + 2);
        terms.push(Term::Open);
        terms.extend(self.terms.iter());
        terms.push(Term::Sep);
        terms.extend(rhs.terms.iter());
        terms.push(Term::Close);
        let mut new = Num { terms };
        new.reduce();
        new
    }
}

impl FromStr for Num {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let terms = s.trim().chars() // This assumes all numbers are < 10 initially
            .map(|c| match c {
                '[' => Term::Open,
                n@('0'..='9') => Term::Num(n.to_digit(10).unwrap() as u32),
                ']' => Term::Close,
                ',' => Term::Sep,
                _ => panic!(),
            }).collect();

        Ok(Self {
            terms
        })
    }
}

impl Debug for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner: String = self.terms.iter()
            .map(|t| match t {
                Term::Num(n) => format!("{}", n),
                Term::Open => "[".to_string(),
                Term::Sep => ",".to_string(),
                Term::Close => "]".to_string(),
            }).collect();
        f.debug_struct("Num").field("terms", &inner).finish()
    }
}

fn main() -> anyhow::Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;

    let input: Vec<Num> = buf.lines()
        .map(|l| l.parse())
        .collect::<Result<_,_>>()?;

    let sum: Num = input.clone().into_iter().fold1(|a, b| a + b).unwrap();

    // println!("sum: {:?}", sum);
    println!("mag: {}", sum.magnitude());

    let len = input.len();

    let max = (0..len).into_par_iter()
        .flat_map(move |i| (0..len).into_par_iter().map(move |j| (i, j)))
        .filter(|(i, j)| i != j)
        .map(|(i, j)| (&input[i] + &input[j]).magnitude())
        .max().unwrap();

    println!("max: {}", max);

    Ok(())
}

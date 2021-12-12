use std::{str::FromStr, io::Read};
use ndarray::prelude::*;


struct OctoMap {
    level: Array2<u8>,
    flashed: Array2<bool>,
}

impl OctoMap {
    pub fn step(&mut self) -> usize {
        self.increase_all();
        let sum = self.start_flash();
        self.reset_all();
        return sum;
    }

    fn increase_all(&mut self) {
        self.level += 1;
    }

    fn reset_all(&mut self) {
        self.level.iter_mut()
            .for_each(|i| if *i > 9 { *i = 0 });
        self.flashed &= false;
    }

    fn start_flash(&mut self) -> usize {
        let (rr,cc) = self.level.dim();
        (0..rr).flat_map(|r| (0..cc).map(move |c| (r, c)))
            .fold(0, |sum, (r, c)| sum + self.flash((r, c)))
    }

    fn flash(&mut self, (r, c): (usize, usize)) -> usize {
        if self.level[[r, c]] > 9 && !self.flashed[[r, c]] {
            self.flashed[[r, c]] = true;
            let mut sum = 1;

            let (rr,cc) = self.level.dim();
            let offsets = [
                (-1, -1), (0, -1), (1, -1),
                (-1,  0),          (1,  0),
                (-1,  1), (0,  1), (1,  1),
            ];
            let neigh = offsets.into_iter().filter_map(|o| {
                let rm = rr as isize;
                let cm = cc as isize;
                let x = r as isize + o.0;
                let y = c as isize + o.1;
                if (0..rm).contains(&x) && (0..cm).contains(&y) {
                    Some((x as usize, y as usize))
                } else {
                    None
                }
            });

            for (r,c) in  neigh {
                self.level[[r,c]] += 1;
                sum += self.flash((r, c));
            }
            sum
        } else {
            0
        }
    }

    pub fn size(&self) -> usize {
        self.level.len()
    }
}


impl FromStr for OctoMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cols = s.lines()
            .nth(0).unwrap()
            .len();
        
        let vals: Vec<u8> = s.lines()
            .map(|l| l.chars())
            .flatten()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        let rows = vals.len() / cols;

        let array = Array2::from_shape_vec((rows, cols), vals)?;

        Ok(Self{
            level: array,
            flashed: Array2::from_shape_simple_fn((rows, cols), || false),
        })
    }
}


fn main() -> anyhow::Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;
    let mut map: OctoMap = buf.parse()?;

    let sum = (0..100).fold(0, |sum, _| sum + map.step());
    println!("{}", sum);

    for i in 100.. {
        if map.step() == map.size() {
            println!("{}", i + 1);
            break;
        }
    }

    Ok(())
}

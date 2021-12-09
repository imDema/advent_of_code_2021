use std::{str::FromStr, io::Read};

struct Floor {
    x: Vec<u8>,
    stride: isize,
}

impl Floor {
    pub fn low_point(&self, idx: usize) -> bool {
        let idx = idx as isize;
        [1, -1, self.stride, -self.stride].into_iter()
            .map(|d| idx + d)
            .filter(|&i| i >= 0 && i < self.x.len() as isize)
            .filter(move |&i| i / self.stride == idx / self.stride || i % self.stride == idx % self.stride)
            .map(|i| self.x[i as usize])
            .all(|x| self.x[idx as usize] < x)
    }

    pub fn sum_lows(&self) -> usize {
        self.x.iter()
            .enumerate()
            .filter_map(|(i, &x)| if self.low_point(i) { Some(x as usize + 1) } else { None })
            .sum()
    }
}

impl FromStr for Floor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stride = s.lines()
            .nth(0).unwrap()
            .len() as isize;
        
        let vals = s.lines()
            .map(|l| l.chars())
            .flatten()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        Ok(Self{
            x: vals,
            stride
        })
    }
}

fn main() -> anyhow::Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;
    let floor: Floor = buf.parse()?;
    let s = floor.sum_lows();

    println!("{}", s);

    Ok(())
}

use std::{str::FromStr, io::Read, collections::VecDeque};

struct Floor {
    x: Vec<Tile>,
    stride: isize,
}

#[derive(Clone, Copy)]
struct Tile {
    depth: u8,
    mark: bool,
}

struct Basin {
    size: usize,
    front: VecDeque<usize>,
}

impl Floor {
    fn neighbours(&self, idx: usize) -> impl Iterator<Item=usize> + '_ {
        let idx = idx as isize;
        [1, -1, self.stride, -self.stride].into_iter()
            .map(move |d| idx + d)
            .filter(|&i| i >= 0 && i < self.x.len() as isize)
            .filter(move |&i| i / self.stride == idx / self.stride || i % self.stride == idx % self.stride)
            .map(|i| i as usize)
    }

    pub fn basins(&mut self) -> Vec<Basin> {
        let mut basins = Vec::new();

        for i0 in 0..self.x.len() {
            let mut b = Basin {
                size: 0,
                front: VecDeque::from(vec![i0])
            };
            while !b.front.is_empty() {
                let p = b.front.pop_front().unwrap();
                if !self.x[p].mark && self.x[p].depth < 9 {
                    // eprintln!("{}:\t{}\t{:?}", i0, p, &b.front);
                    self.x[p].mark = true;
                    b.size += 1;
                    b.front.extend(self.neighbours(p));
                }
            }
            if b.size > 0 {
                println!("{}: {}", i0, b.size);
                basins.push(b);
            }
        }

        basins
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
            .map(|depth| Tile{ depth, mark: false })
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
    let mut floor: Floor = buf.parse()?;
    let mut basins = floor.basins();
    basins.sort_unstable_by(|a, b| b.size.cmp(&a.size));
    let result = basins.into_iter().take(3)
        .fold(1, |acc, x| acc * x.size);

    println!("{}", result);

    Ok(())
}

use std::{str::FromStr, io::Read, collections::{BinaryHeap, HashSet}, cmp::Reverse};
use ndarray::prelude::*;

struct Map {
    pub risk: Array2<u8>,
}

impl Map {
    fn djikstra(&self) -> u32 {
        let dims = self.risk.dim();
        let mut cost = Array2::from_shape_simple_fn(dims, || u32::MAX);
        cost[[0, 0]] = 0;
        let mut queue = BinaryHeap::new();
        let mut visited = HashSet::new();
        let end = [dims.0-1, dims.1-1];
        queue.push((Reverse(0), [0, 0]));
        while let Some((_, pos)) = queue.pop() {
            if pos == end { break; }
            if !visited.insert(pos) { continue; }
            for neigh in self.neigh(pos).filter(|c| !visited.contains(c)) {
                let cv = cost[pos] + self.risk[neigh] as u32;
                if cv < cost[neigh] {
                    cost[neigh] = cv;
                    queue.push((Reverse(cv), neigh));
                }
            }
        }
        cost[end]
    }

    fn neigh(&self, [i, j]: [usize; 2]) -> impl Iterator<Item=[usize; 2]>{
        let dims = self.risk.dim();
        let i = i as isize;
        let j = j as isize;
        [[1, 0],[-1, 0],[0, 1],[0,-1]].into_iter()
            .map(move |[di, dj]| [i + di, j + dj])
            .filter(move |&[i, j]| i >= 0 && j >= 0 && i < dims.0 as isize && j < dims.1 as isize)
            .map(|[i, j]| [i as usize, j as usize])
    }

    pub fn tile_5x5(&self) -> Map {
        let (ii, jj) = self.risk.dim();
        let cost = Array2::from_shape_fn((ii*5, jj*5), |(i, j)|
            ((self.risk[[i % ii, j % jj]] as usize + i / ii + j / jj - 1) % 9 + 1) as u8
        );
        Map {
            risk: cost
        }
    }
}


impl FromStr for Map {
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
            risk: array,
        })
    }
}


fn main() -> anyhow::Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;

    let map: Map = buf.parse()?;
    // map.print();
    println!("{}", map.djikstra());

    let map5 = map.tile_5x5();
    // map5.print();
    println!("{}", map5.djikstra());

    Ok(())
}

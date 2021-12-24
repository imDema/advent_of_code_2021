use std::collections::HashSet;
use std::ops::Range;
use std::io::Read;
use rayon::prelude::*;

use plotters::prelude::*;

type LUT = Vec<bool>;
type Coord = (i32, i32);

pub fn parse_lut(s: &str) -> LUT {
    let values = s.chars()
        .filter(|c| *c != '\n')
        .map(|c| match c {
            '#' => true,
            '.' => false,
            _ => panic!("unexpected char in LUT"),
        }).collect();
    values
}

struct Img {
    pix: HashSet<Coord>,
    bb: BoundingBox,
    flipped: bool,
}

impl Img {
    pub fn parse(s: &str) -> anyhow::Result<Self> {
        let mut pix = HashSet::new();
        let mut bb = BoundingBox::new((0, 0));
        for (j, l) in s.lines().enumerate() {
            for (i, c) in l.chars().enumerate() {
                match c {
                    '#' => {
                        let p = (i as i32, j as i32);
                        pix.insert(p);
                        bb.extend(p);
                    }
                    '.' => {}
                    _ => panic!("Invalid char in image {}", c),
                }
            }
        }
        Ok(Self { pix, bb, flipped: false })
    }

    pub fn count(&self) -> usize {
        self.pix.len()
    }

    pub fn plot(&self, path: &str) -> anyhow::Result<()> {
        eprintln!();
        for j in self.bb.y.clone() {
            for i in self.bb.x.clone() {
                if self.pix.contains(&(i,j)) {
                    eprint!("#");
                } else {
                    eprint!(".");
                }
            }
            eprintln!();
        }
        eprintln!();

        let root = BitMapBackend::new(path, (640, 480)).into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .build_cartesian_2d(self.bb.x.clone(), self.bb.y.clone())?;

        chart.configure_mesh().draw()?;

        chart.draw_series(
            self.pix.iter().map(|c| {
                Circle::new(*c, 2, RED)
            }))?;

        Ok(())
    }

    pub fn convolve(&self, lut: &LUT) -> Self {
        let mut n: Self = self.bb.scan_padded(2)
            .filter(|c| self.flipped  ^ self.conv_val(*c, lut))
            .collect();

        n.flipped = !self.flipped;
        n
    }

    fn conv_val(&self, center: Coord, lut: &LUT) -> bool {
        let idx = self.conv_set(center)
            .fold(0, |acc, x| (acc << 1) | x as usize);
        if self.flipped {
            lut[!idx % (1<<9)]
        } else {
            lut[idx]
        }
    }

    fn conv_set(&self, center: Coord) -> impl Iterator<Item=bool> + '_ {
        [-1, 0, 1].into_iter()
            .flat_map(|y| [-1, 0, 1].into_iter().map(move |x| (x, y)))
            .map(move |(dx, dy)| (center.0 + dx, center.1 + dy))
            .map(|p| self.pix.contains(&p))
    }
}

impl FromParallelIterator<Coord> for Img {
    fn from_par_iter<T: IntoParallelIterator<Item = Coord>>(iter: T) -> Self {
        let pix: HashSet<_> = iter.into_par_iter().collect();

        let bb = pix.iter()
            .fold(BoundingBox::new((0,0)), |mut bb, p| {
                bb.extend(*p);
                bb
            });

        Self {
            pix,
            bb,
            flipped: false,
        }
    }
}

struct BoundingBox {
    x: Range<i32>,
    y: Range<i32>,
}

impl BoundingBox {
    pub fn new(p: Coord) -> Self {
        Self {
            x: p.0..p.0+1,
            y: p.1..p.1+1,
        }
    }

    pub fn extend(&mut self, p: Coord) {
        self.x.start = self.x.start.min(p.0);
        self.x.end = self.x.end.max(p.0 + 1);
        self.y.start = self.y.start.min(p.1);
        self.y.end = self.y.end.max(p.1 + 1);
    }

    pub fn scan_padded(&self, padding: i32) -> impl ParallelIterator<Item=Coord> + '_ {
        assert!(padding >= 0);
        let x = self.x.start-padding..self.x.end+padding;
        let y = self.y.start-padding..self.y.end+padding;
        x.into_par_iter()
            .flat_map(move |x| y.clone().into_par_iter().map(move |y| (x, y)))
    }
}

fn main() -> anyhow::Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;

    let (lut, img) = buf.split_once("\n\n").unwrap();
    let lut = parse_lut(lut);
    let mut img = Img::parse(img)?;

    for i in 0..2 {
        img = img.convolve(&lut);
        img.plot(&format!("day20/plot_{}.png", i))?;
    }

    println!("{}", img.count());

    Ok(())
}

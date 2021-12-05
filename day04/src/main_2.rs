use std::io::Read;

struct Cell {
    pub n: u32,
    pub mark: bool,
}

impl Cell {
    pub fn new(n: u32) -> Self {
        Self {n, mark: false}
    }
}

struct Board {
    cells: Vec<Cell>,
    w: usize,
}

impl Board {
    pub fn parse(s: &str) -> anyhow::Result<Self> {
        let cells= s.split_whitespace()
            .filter(|w| w.len() > 0)
            .map(|w| u32::from_str_radix(w, 10).map(|n| Cell::new(n)))
            .collect::<Result<Vec<_>, _>>()?;

        let w = (cells.len() as f32).sqrt().round() as usize;
        
        assert_eq!(cells.len(), w * w);
        Ok(Self { cells, w })
    }

    pub fn mark(&mut self, n: u32) -> Option<u32> {
        for i in 0..self.cells.len() {
            if self.cells[i].n == n {
                self.cells[i].mark = true;
                let (c, r) = (i % self.w, i / self.w);
                let row_win = self.cells.iter().skip(r*self.w).take(self.w).all(|c| c.mark);
                let col_win = self.cells.iter().skip(c).step_by(self.w).all(|c| c.mark);
                if row_win || col_win {
                    return Some(self.board_score())
                }
            }
        }
        None
    }

    fn board_score(&self) -> u32 {
        self.cells.iter()
            .filter(|c| !c.mark)
            .map(|c| c.n)
            .sum()
    }
}

fn main() -> anyhow::Result<()> {
    let mut stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf)?;

    let rolls: Vec<u32> = buf
        .split(',')
        .map(|n| u32::from_str_radix(n.trim_end(), 10).unwrap())
        .collect();

    buf.truncate(0);
    stdin.read_to_string(&mut buf)?;

    let mut boards = buf.split("\n\n")
        .filter(|s| s.len() > 0)
        .map(|s| Board::parse(s))
        .collect::<Result<Vec<_>,_>>()?;

    for r in rolls {
        let mut i = 0;
        while i < boards.len() {
            if let Some(score) = boards[i].mark(r) {
                boards.swap_remove(i);
                if boards.is_empty() {
                    println!("score: {}, roll: {}, res: {}", score, r, score * r);
                    break;
                }
            } else {
                i += 1;
            }
        }
    }

    Ok(())
}

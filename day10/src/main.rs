use std::io::Read;

struct Parser {
    stack: Vec<char>,
}

impl Parser {
    pub fn new() -> Self { Self { stack: Vec::new() } }

    pub fn step(&mut self, c: char) -> Result<(), usize> {
        match c {
            '(' | '[' | '{' | '<' => {
                self.stack.push(c);
                Ok(())
            }
            ')' | ']' | '}'| '>' => {
                if let Some(head) = self.stack.pop() {
                    match (head, c) {
                        ('(', ')') => Ok(()),
                        ('[', ']') => Ok(()),
                        ('{', '}') => Ok(()),
                        ('<', '>') => Ok(()),
                        (_, ')') => Err(3),
                        (_, ']') => Err(57),
                        (_, '}') => Err(1197),
                        (_, '>') => Err(25137),
                        _ => unreachable!(),
                    }
                } else {
                    panic!("Empty stack!");
                }
            }
            _ => panic!("Invalid characters"),
        }
    }

    pub fn flush(&mut self) -> Vec<char> {
        self.stack.drain(..).rev().collect()
    }
}

fn main() -> anyhow::Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;
    let mut mismatch_score = 0;
    let mut complete_score: Vec<u64> = Vec::new();
    for l in buf.lines() {
        let mut parser = Parser::new();
        let errval = l.chars()
            .map(|c| parser.step(c))
            .find(|r| r.is_err())
            .map(|e| e.unwrap_err());
        if let Some(e) = errval {
            mismatch_score += e;
        } else {
            let score = parser.flush().into_iter()
                .fold(0, |acc, c| {
                    acc * 5 + match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => panic!("Invalid char in stack"),
                    }
                });
            complete_score.push(score);
        }
    }

    complete_score.sort_unstable();
    let complete_median = complete_score[complete_score.len() / 2];

    println!("mismatch: {}", mismatch_score);
    println!("completion: {}", complete_median);

    Ok(())
}

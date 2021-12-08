use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let count: usize = stdin.lock().lines()
        .map(|l| l.unwrap())
        .map(|l| l.split('|')
            .nth(1).unwrap()
            .split_whitespace()
            .filter(|w| match w.len() {
                2 | 3 | 4 | 7 => true,
                _ => false,
            }).count())
        .sum();

    println!("c: {}", count);
}

use std::io::Read;

fn compute_distance(v: &Vec<i32>, pos: i32) -> i32 {
    v.iter()
        .map(|x| (x - pos).abs())
        .sum()
}

fn main() -> anyhow::Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;

    let mut input: Vec<i32> = buf.split(',')
        .map(|w| w.parse().unwrap())
        .collect();

    input.sort_unstable();
    
    let h1 = input[input.len() / 2];
    let h2 = input[input.len() / 2 + 1];

    let d1 = compute_distance(&input, h1);
    let d2 = compute_distance(&input, h2);

    println!("h1: {}\td1: {}", h1, d1);
    println!("h2: {}\td2: {}", h2, d2);


    Ok(())
}

use std::io::Read;

fn main() -> anyhow::Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;

    let depths: Vec<u32> = buf.lines()
        .map(|l| u32::from_str_radix(l, 10))
        .collect::<Result<_,_>>()?;

    let windows: Vec<u32> = depths.windows(3)
        .map(|w| w.iter().sum())
        .collect();

    let c = windows.windows(2)
        .filter(|w| w[0] < w[1])
        .count();

    println!("{}", c);

    Ok(())
}

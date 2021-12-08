use std::io::Read;

fn f(v: &Vec<i64>, pos: i64) -> i64 {
    v.iter()
        .map(|x| {
            let d = (x - pos).abs();
            d * (d + 1) / 2
        })
        .sum()
}

fn df(v: &Vec<i64>, pos: i64) -> i64 {
    v.iter().map(|x| pos - x).sum()
}

fn bisection<F: Fn(i64) -> i64>(f: F, (a, b): (i64, i64)) -> i64 {
    let mut a = a;
    let mut b = b;
    let mut fa = f(a);
    let mut fb = f(b);
    while a + 1 < b {
        let x = (a + b) / 2;
        let fx = f(x);
        println!("x: {}, fx: {}", x, fx);
        match fx.cmp(&0) {
            std::cmp::Ordering::Less => {
                a = x;
                fa = fx;
            }
            std::cmp::Ordering::Greater => {
                b = x;
                fb = fx;
            }
            std::cmp::Ordering::Equal => {
                return x;
            }
        }
    }
    if a == b {
        return a;
    } else if a == b - 1 {
        return if fa.abs() < fb.abs() {a} else {b};
    }
    panic!();
}

fn main() -> anyhow::Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;

    let input: Vec<i64> = buf.split(',')
        .map(|w| w.parse().unwrap())
        .collect();

    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    let min = bisection(|p| df(&input, p), (min, max));

    let d1 = f(&input, min);

    println!("min: {}, dist: {}", min, d1);


    Ok(())
}

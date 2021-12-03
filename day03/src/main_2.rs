use std::io::BufRead;

type BitVec = Vec<bool>;

fn parse_bits(s: &str) -> BitVec {
    s.chars()
        .map(|c| match c {
            '1' => true,
            '0' => false,
            _ => panic!(),
        })
        .collect()
}

struct Cohorts {
    mc: Vec<BitVec>,
    lc: Vec<BitVec>,
}

/// Split popolation in most common and least common cohort
/// based on the value at `idx`
fn cohorts(pop: Vec<BitVec>, idx: usize) -> Cohorts {
    let mut mc = pop;
    let mut lc = Vec::new();

    let ones = mc.iter()
        .filter(|&v| v[idx])
        .count();

    let one_dominant = ones >= (mc.len() + 1 )/ 2;

    let mut i = 0;
    while i < mc.len() {
        if one_dominant && mc[i][idx] || !one_dominant && !mc[i][idx] {
            i += 1;
        } else {
            lc.push(mc.swap_remove(i));
        }
    }
    Cohorts { mc, lc }
}

fn parse_bitvec(b: &BitVec) -> usize {
    b.into_iter().fold(0, |acc, x| {
        (acc << 1) | if *x { 1 } else { 0 }
    })
}

fn main() {
    let stdin = std::io::stdin();

    let bits: Vec<_> = stdin.lock().lines()
        .map(|l| parse_bits(&l.unwrap()))
        .collect();

    let len = bits[0].len();

    let ch = Cohorts {
        mc: bits.clone(),
        lc: bits.clone(),
    };
    let result = (0..len).fold(ch ,|acc, idx| { // Does not terminate early
        let mc = if acc.mc.len() > 1 {
            cohorts(acc.mc, idx).mc
        } else {
            acc.mc
        };
        let lc = if acc.lc.len() > 1 {
            cohorts(acc.lc, idx).lc
        } else {
            acc.lc
        };
        Cohorts {mc, lc}
    });

    assert_eq!(result.mc.len(), 1);
    assert_eq!(result.lc.len(), 1);

    let o2 = parse_bitvec(&result.mc[0]);
    let co2 = parse_bitvec(&result.lc[0]);

    println!("O2: {}, CO2: {}, res: {}", o2, co2, o2*co2);
}

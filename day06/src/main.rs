use std::collections::VecDeque;

const YOUNG_DELAY: usize = 8;
const ADULT_DELAY: usize = 6;

struct School {
    pub fish: VecDeque<usize>,
}

impl School {
    pub fn new(fish: Vec<usize>) -> Self {
        Self {
            fish: VecDeque::from(fish),
        }
    }

    pub fn spin(&mut self) {
        assert_eq!(self.fish.len(), YOUNG_DELAY + 1);
        let spawn = self.fish.pop_front().unwrap();
        self.fish.push_back(spawn);
        self.fish[ADULT_DELAY] += spawn;
    }
}

fn main() -> anyhow::Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;
    
    let fish = buf[..buf.len()-1].split(',')
        .map(|n| n.parse().unwrap())
        .fold(vec![0; YOUNG_DELAY + 1], |mut v, x: usize| {
            assert!(x <= YOUNG_DELAY);
            v[x] += 1;
            v
        });

    let mut school = School::new(fish);

    for _ in 0..80 {
        school.spin();
    }

    let count: usize = school.fish.iter().sum();
    println!("80.\tfish: {}", count);

    for _ in 80..256 {
        school.spin();
    }

    let count: usize = school.fish.iter().sum();
    println!("256.\tfish: {}", count);

    Ok(())
}

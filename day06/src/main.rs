const SIZE: usize = 9;
const OFFSET: usize = 2;

struct School {
    pub fish: [usize; SIZE],
}

impl School {
    pub fn spin(&mut self) {
        let spawn = self.fish[0];
        for i in 1..self.fish.len() {
            self.fish[i-1] = self.fish[i];
        }
        self.fish[SIZE - 1] = spawn;
        self.fish[SIZE - 1 - OFFSET] += spawn;
    }
}

fn main() -> anyhow::Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;
    
    let fish = buf[..buf.len()-1].split(',')
        .map(|n| n.parse().unwrap())
        .fold([0; SIZE], |mut v, x: usize| {
            assert!(x < SIZE);
            v[x] += 1;
            v
        });

    let mut school = School{ fish };

    for _ in 0..80 {
        school.spin();
    }

    let count: usize = school.fish.into_iter().sum();
    println!("80.\tfish: {}", count);

    for _ in 80..256 {
        school.spin();
    }

    let count: usize = school.fish.into_iter().sum();
    println!("256.\tfish: {}", count);

    

    Ok(())
}

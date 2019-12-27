const DIMENSIONS: usize = 3;
const STEPS: usize = 1000;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Moon {
    position: [i64; DIMENSIONS],
    velocity: [i64; DIMENSIONS],
}

impl Moon {
    fn potential_energy(&self) -> i64 {
        self.position.iter().copied().map(i64::abs).sum()
    }

    fn kinetic_energy(&self) -> i64 {
        self.velocity.iter().copied().map(i64::abs).sum()
    }

    fn total_energy(&self) -> i64 {
        self.potential_energy() * self.kinetic_energy()
    }
}

fn main() {
    let mut moons = include_str!("input.txt")
        .lines()
        .map(|line| {
            let mut xyz = line[1..line.len() - 1]
                .split(", ")
                .map(|p| p[2..].parse().unwrap());

            let x = xyz.next().unwrap();
            let y = xyz.next().unwrap();
            let z = xyz.next().unwrap();

            Moon {
                position: [x, y, z],
                velocity: [0; DIMENSIONS],
            }
        })
        .collect::<Vec<_>>();

    let initial_moons = moons.clone();

    let mut cycles = [0; DIMENSIONS];
    let mut remaining = DIMENSIONS;

    let mut step = 0;
    while remaining != 0 {
        if step == STEPS {
            println!("{}", moons.iter().map(Moon::total_energy).sum::<i64>());
        }

        for k in 0..DIMENSIONS {
            if cycles[k] != 0 {
                continue;
            }

            for i in 0..moons.len() {
                for j in 0..i {
                    let delta = (moons[i].position[k] - moons[j].position[k]).signum();
                    moons[i].velocity[k] -= delta;
                    moons[j].velocity[k] += delta;
                }
            }

            for i in 0..moons.len() {
                moons[i].position[k] += moons[i].velocity[k];
            }
        }

        step += 1;

        for k in 0..DIMENSIONS {
            if cycles[k] == 0
                && (0..moons.len()).all(|i| {
                    moons[i].position[k] == initial_moons[i].position[k]
                        && moons[i].velocity[k] == 0
                })
            {
                cycles[k] = step;
                remaining -= 1;
            }
        }
    }

    println!("lcm {:?}", cycles);
}

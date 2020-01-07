const MOONS: usize = 4;
const STEPS: usize = 1000;

fn simulate_axis(initial_positions: [i64; MOONS]) -> (([i64; MOONS], [i64; MOONS]), usize) {
    let mut positions = initial_positions;
    let mut velocities = [0; MOONS];

    let mut part1 = (positions, velocities);

    let mut n = 1;
    'find_cycle: loop {
        for i in 0..positions.len() {
            for j in 0..i {
                let delta = positions[i].cmp(&positions[j]) as i64;
                velocities[i] -= delta;
                velocities[j] += delta;
            }
        }

        for i in 0..positions.len() {
            positions[i] += velocities[i];
        }

        if n == STEPS {
            part1 = (positions, velocities);
        }

        if (0..MOONS).all(|i| positions[i] == initial_positions[i] && velocities[i] == 0) {
            break 'find_cycle;
        }

        n += 1;
    }

    (part1, n)
}

fn main() {
    let mut x_axis = [0; MOONS];
    let mut y_axis = [0; MOONS];
    let mut z_axis = [0; MOONS];

    include_str!("input.txt")
        .lines()
        .enumerate()
        .for_each(|(i, line)| {
            let mut xyz = line[1..line.len() - 1]
                .split(", ")
                .map(|p| p[2..].parse().unwrap());

            let x = xyz.next().unwrap();
            let y = xyz.next().unwrap();
            let z = xyz.next().unwrap();

            x_axis[i] = x;
            y_axis[i] = y;
            z_axis[i] = z;
        });

    let ((x, vx), x_cycle) = simulate_axis(x_axis);
    let ((y, vy), y_cycle) = simulate_axis(y_axis);
    let ((z, vz), z_cycle) = simulate_axis(z_axis);

    println!(
        "{}",
        (0..MOONS)
            .map(|i| (x[i].abs() + y[i].abs() + z[i].abs())
                * (vx[i].abs() + vy[i].abs() + vz[i].abs()))
            .sum::<i64>()
    );
    println!("lcm {} {} {}", x_cycle, y_cycle, z_cycle);
}

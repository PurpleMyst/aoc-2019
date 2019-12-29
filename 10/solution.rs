use std::{
    collections::HashSet,
    f64::consts::{FRAC_PI_2, PI},
};

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Fraction(i64, i64);

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

impl Fraction {
    fn simplify(&mut self) {
        if self.0 == 0 && self.1 == 0 {
            return;
        }

        let d = gcd(self.0.abs(), self.1.abs());
        self.0 /= d;
        self.1 /= d;
    }
}

/// Calculate the clockwise angle from the positive y axis from one point to another
fn angle_between((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> f64 {
    let mut theta = FRAC_PI_2 - f64::atan2((y2 - y1) as f64, (x2 - x1) as f64);

    if theta < 0.0 {
        theta += 2.0 * PI;
    }

    theta
}

fn main() {
    let mut asteroids = include_str!("input.txt")
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.bytes().enumerate().filter_map(move |(x, c)| {
                if c == b'#' {
                    // invert the y coordinate to use atan2 correctly
                    Some((x as i64, -(y as i64)))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();

    // To find the monitoring station, count the number of unique slopes around each point and find
    // the asteroid with the most.
    let (part1, i) = asteroids
        .iter()
        .enumerate()
        .map(|(i, (x1, y1))| {
            (
                asteroids[..i]
                    .iter()
                    .copied()
                    .chain(asteroids[i + 1..].iter().copied())
                    .map(|(x2, y2)| {
                        let mut slope = Fraction(y2 - y1, x2 - x1);
                        slope.simplify();
                        slope
                    })
                    .collect::<HashSet<_>>()
                    .len(),
                i,
            )
        })
        .max()
        .unwrap();

    println!("{}", part1);

    let station = asteroids.remove(i);

    // Construct a list of targets grouped by angle
    let mut targets: Vec<(f64, Vec<(i64, i64)>)> = vec![];
    for asteroid in asteroids {
        let alpha = angle_between(station, asteroid);

        match targets.binary_search_by(|(beta, _)| alpha.partial_cmp(beta).unwrap()) {
            Ok(idx) => targets[idx].1.push(asteroid),
            Err(idx) => targets.insert(idx, (alpha, vec![asteroid])),
        }
    }
    targets.reverse();

    // Iterate through the targets list multiple times, only removing one per group per iteration
    let mut vaporized = 0;
    loop {
        for (_, group) in &mut targets {
            let (x, y) = group.pop().unwrap();
            vaporized += 1;
            if vaporized == 200 {
                println!("{}", 100 * x - y);
                return;
            }
        }
    }
}

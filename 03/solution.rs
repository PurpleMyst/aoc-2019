use std::{cmp::min, collections::HashMap};

fn points(path: &str) -> Vec<(isize, isize)> {
    let mut result = Vec::new();

    let mut x = 0;
    let mut y = 0;

    for step in path.split(',') {
        let mut step = step.chars();

        let direction = step.next().unwrap() as u8;
        let amount = step.as_str().parse().unwrap();

        result.reserve(amount);
        for _ in 0..amount {
            match direction {
                b'R' => x += 1,
                b'L' => x -= 1,
                b'U' => y += 1,
                b'D' => y -= 1,
                _ => unreachable!(),
            }

            result.push((x, y));
        }
    }

    result
}

fn main() {
    let mut wires = include_str!("input.txt").trim().lines().map(points);

    let wire1 = wires.next().unwrap();
    let wire2 = wires.next().unwrap();

    let wire1 = wire1
        .into_iter()
        .enumerate()
        .map(|(k, v)| (v, k))
        .collect::<HashMap<_, _>>();

    let mut part1 = 2_000;
    let mut part2 = 200_000;

    wire2.into_iter().enumerate().for_each(|(steps2, point)| {
        if let Some(steps1) = wire1.get(&point) {
            part1 = min(part1, point.0.abs() + point.1.abs());
            part2 = min(part2, steps1 + steps2);
        }
    });

    println!("{}\n{}", part1, part2 + 2);
}

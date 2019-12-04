use std::iter;

fn points(path: &str) -> impl Iterator<Item = (isize, isize)> + Clone + '_ {
    path.split(',')
        .map(|step| step.chars())
        .map(|mut step| (step.next().unwrap(), step.as_str().parse().unwrap()))
        .flat_map(|(direction, amount)| iter::repeat(direction).take(amount))
        .scan((0, 0), |pos, direction| {
            match direction {
                'R' => pos.0 += 1,
                'L' => pos.0 -= 1,
                'U' => pos.1 += 1,
                'D' => pos.1 -= 1,
                _ => unreachable!(),
            };
            Some(*pos)
        })
}

fn main() {
    use std::collections::{HashMap, HashSet};

    let mut wires = include_str!("input.txt").trim().lines().map(points);

    let wire1 = wires.next().unwrap();
    let wire2 = wires.next().unwrap();

    // Part 1
    println!(
        "{}",
        wire1
            .clone()
            .collect::<HashSet<_>>()
            .intersection(&wire2.clone().collect())
            .map(|(x, y)| x.abs() + y.abs())
            .min()
            .unwrap()
    );

    // Part 2
    let wire1 = wire1.enumerate().map(|(k, v)| (v, k + 1));
    let wire2 = wire2
        .enumerate()
        .map(|(k, v)| (v, k + 1))
        .collect::<HashMap<_, _>>();

    println!(
        "{}",
        wire1
            .filter_map(|(k, v)| Some(v + wire2.get(&k)?))
            .min()
            .unwrap()
    );
}

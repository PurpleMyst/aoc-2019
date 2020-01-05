fn fft(signal: &[i64], idx: usize) -> i64 {
    use std::cmp::min;

    let mut total: i64 = 0;

    for i in (idx..signal.len()).step_by(4 * (idx + 1)) {
        for j in i..min(signal.len(), i + (idx + 1)) {
            total += signal[j];
        }
    }

    for i in (3 * (idx + 1) - 1..signal.len()).step_by(4 * (idx + 1)) {
        for j in i..min(signal.len(), i + idx + 1) {
            total -= signal[j];
        }
    }

    total.abs() % 10
}

fn step(signal: &[i64]) -> Vec<i64> {
    let mut next = signal.to_vec();
    for i in 0..signal.len() {
        next[i] = fft(signal, i);
    }
    next
}

fn step2(signal: &mut [i64]) {
    let mut acc = 0;

    signal.iter_mut().rev().for_each(|elem| {
        acc = (acc + *elem) % 10;
        *elem = acc;
    });
}

fn main() {
    let mut signal_part1 = include_str!("input.txt")
        .trim()
        .bytes()
        .map(|c| (c - b'0') as i64)
        .collect::<Vec<_>>();

    let mut signal_part2 = signal_part1.clone();
    (0..10000 - 1).for_each(|_| signal_part2.extend_from_slice(&signal_part1));

    // part 1
    (0..100).for_each(|_| signal_part1 = step(&signal_part1[..]));
    signal_part1[..8].iter().for_each(|c| print!("{}", c));
    println!();

    // part 2
    let offset = signal_part2[..7].iter().fold(0, |acc, d| 10 * acc + d) as usize;
    signal_part2.drain(..offset);
    (0..100).for_each(|_| step2(&mut signal_part2));
    signal_part2[..8].iter().for_each(|c| print!("{}", c));
    println!();
}

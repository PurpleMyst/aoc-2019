use std::{cmp::min, mem::transmute};

const INPUT_LEN: usize = 650;
const REPETITION: usize = 10_000;

fn digit(signal: [i32; INPUT_LEN], idx: usize) -> i32 {
    let mut total: i32 = 0;

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

fn fft_part1(signal: [i32; INPUT_LEN]) -> [i32; INPUT_LEN] {
    let mut next = [0; INPUT_LEN];
    // We utilize `iter_mut().enumerate()` to avoid bounds checking
    for (i, elem) in next.iter_mut().enumerate() {
        *elem = digit(signal, i);
    }
    next
}

// At sufficiently high values, namely when the index is greater than half of the signal length,
// the above FFT function reduces to this
fn fft_part2(signal: &mut [u32]) {
    let mut acc = 0;

    for n in signal.iter_mut().rev() {
        acc += *n;
        *n = acc;
    }

    for n in signal.iter_mut() {
        *n %= 10;
    }
}

fn main() {
    let mut signal = [0; INPUT_LEN];

    include_bytes!("input.txt")
        .iter()
        .copied()
        .enumerate()
        .for_each(|(i, c)| signal[i] = (c - b'0') as i32);

    let offset = signal[..7]
        .iter()
        .fold(0usize, |acc, d| 10 * acc + (*d as usize));

    // Transmuting &[i32] to &[u32] is perfectly safe because they have the same size
    let unsigned_signal: &[u32] = unsafe { transmute(&signal[..]) };

    // Calculate the first repetition which actually appears
    // after the offset and copy starting from that to avoid superfluous copying.
    let mut repeated_signal = Vec::with_capacity(INPUT_LEN * REPETITION - offset);
    let mut it = (0..REPETITION).skip_while(|i| (i + 1) * INPUT_LEN < offset);
    repeated_signal.extend_from_slice(&unsigned_signal[offset - it.next().unwrap() * INPUT_LEN..]);
    it.for_each(|_| repeated_signal.extend_from_slice(unsigned_signal));

    // part 1
    (0..100).for_each(|_| signal = fft_part1(signal));
    signal[..8].iter().for_each(|c| print!("{}", c));
    println!();

    // part 2
    (0..100).for_each(|_| fft_part2(&mut repeated_signal[..]));
    repeated_signal[..8].iter().for_each(|c| print!("{}", c));
    println!();
}

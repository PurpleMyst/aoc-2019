use std::{
    cmp::min,
    io::{self, Write},
    mem::transmute,
};

const INPUT_LEN: usize = 650;
const REPETITION: usize = 10_000;

const ITERATIONS: u32 = 100;

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

fn binom_mod2(m: u32, n: u32) -> u32 {
    ((!m & n) == 0) as u32
}

fn binom_smol(m: u32, n: u32) -> u32 {
    if m < n {
        return 0;
    }

    let mut res = 1;
    for i in 0..n {
        res *= m - i;

        if res == 0 {
            break;
        }

        res /= i + 1;
    }
    res
}

fn binom_mod5(mut m: u32, mut n: u32) -> u32 {
    let mut res = 1;

    while m != 0 && n != 0 && res != 0 {
        let d1 = m % 5;
        let d2 = n % 5;

        res *= binom_smol(d1, d2);
        res %= 5;

        m /= 5;
        n /= 5;
    }

    res
}

fn binom_mod10(m: u32, n: u32) -> u32 {
    if m < n {
        return 0;
    }

    let m2 = binom_mod2(m, n);
    let m5 = binom_mod5(m, n);

    (if m2 == 0 {
        // 0 % 5 = 0
        // 2 % 5 = 2
        // 4 % 5 = 4
        // 6 % 5 = 1
        // 8 % 5 = 3
        [0, 6, 2, 8, 4]
    } else {
        // 1 % 5 = 1
        // 3 % 5 = 3
        // 5 % 5 = 0
        // 7 % 5 = 2
        // 9 % 5 = 4
        [5, 1, 7, 3, 9]
    })[m5 as usize]
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
    // TODO: only remaining optimization is to remove the repetition vector completely and just
    // work on the `unsigned_signal`
    let mut repeated_signal = Vec::with_capacity(INPUT_LEN * REPETITION - offset);
    let mut it = (0..REPETITION).skip_while(|i| (i + 1) * INPUT_LEN < offset);
    repeated_signal.extend_from_slice(&unsigned_signal[offset - it.next().unwrap() * INPUT_LEN..]);
    it.for_each(|_| repeated_signal.extend_from_slice(unsigned_signal));

    let stdout = io::stdout();
    let mut lock = stdout.lock();

    // part 1
    (0..ITERATIONS).for_each(|_| signal = fft_part1(signal));
    signal[..8]
        .iter()
        .try_for_each(|&c| write!(lock, "{}", c))
        .unwrap();
    writeln!(lock).unwrap();

    let coeffs = (0..(INPUT_LEN * REPETITION - offset) as u32)
        .map(|i| binom_mod10(ITERATIONS - 1 + i, i))
        .collect::<Vec<_>>();

    for k in 0..8 {
        let repeated_signal = &repeated_signal[k..];

        let d = repeated_signal
            .iter()
            .zip(coeffs.iter().copied())
            .map(|(a, b)| a * b)
            .sum::<u32>()
            % 10;

        write!(lock, "{}", d).unwrap();
    }
}

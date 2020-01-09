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

// Calculate mCn (mod 2) using some bitwise trickery
fn binom_mod2(m: u32, n: u32) -> u32 {
    ((!m & n) == 0) as u32
}

// Calculate mCn (mod 5) using Lucas's theorem
fn binom_mod5(mut m: u32, mut n: u32) -> u32 {
    // Precalculated array of all possible binomials for m and n less than 5
    const LOOKUP_TABLE: [u32; 5 * 5] = [
        1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 2, 1, 0, 0, 1, 3, 3, 1, 0, 1, 4, 1, 4, 1,
    ];

    let mut res = 1;
    while m != 0 && n != 0 && res != 0 {
        res *= LOOKUP_TABLE[(5 * (m % 5) + (n % 5)) as usize];
        res %= 5;
        m /= 5;
        n /= 5;
    }
    res
}

// Calculate mCn (mod 10) using the Chinese Remainder Theorem
fn binom_mod10(m: u32, n: u32) -> u32 {
    if m < n {
        return 0;
    }

    let m2 = binom_mod2(m, n);
    let m5 = binom_mod5(m, n);

    // To utilize the CRT, we must find an integer satisfying two linear congruences
    // 1) x = m2 (mod 2)
    // 2) x = m5 (mod 5)
    // Since the first equation is modulo two, there are only two possibilities for m2, zero and
    // one, so we can already restrict the search space in half by considering evens/odds. Then, we
    // order the considered numbers by their remainder modulo 5 and index into it like a lookup table.
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
    let stdout = io::stdout();
    let mut lock = stdout.lock();

    let mut signal = [0; INPUT_LEN];

    const INPUT: [u8; INPUT_LEN] = *include_bytes!("input.txt");

    // Copy the input into the signal as i32 integers
    signal
        .iter_mut()
        .zip(INPUT.iter())
        .for_each(|(elem, value)| *elem = (value - b'0') as i32);

    // Calculate the offset manually
    const OFFSET: usize = 0
        + 1 * (INPUT[6] - b'0') as usize
        + 10 * (INPUT[5] - b'0') as usize
        + 100 * (INPUT[4] - b'0') as usize
        + 1000 * (INPUT[3] - b'0') as usize
        + 10000 * (INPUT[2] - b'0') as usize
        + 100000 * (INPUT[1] - b'0') as usize
        + 1000000 * (INPUT[0] - b'0') as usize;

    // Transmuting &[i32] to &[u32] is perfectly safe because they have the same size
    let unsigned_signal: [u32; INPUT_LEN] = unsafe { transmute(signal) };

    // Solve part 1 by running the FFT algorithm on the signal
    (0..ITERATIONS).for_each(|_| signal = fft_part1(signal));
    signal[..8]
        .iter()
        .try_for_each(|&c| write!(lock, "{}", c))
        .unwrap();
    writeln!(lock).unwrap();

    // Solve part 2 in a slightly smarter way by noticing that the FFT algorithm corresponds to
    // summing in reverse if the offset is large enough. This is then equivalent to multiplying a
    // vector by a matrix raised to the 100th power, and this particular matrix is an unitriangular
    // matrix, which are defined to have only ones above the main diagonal. A particular property
    // of this kind of matrix is that their exponetiation is given by the Nth diagonal on Pascal's
    // triangle, which can be calculated with binomial coefficients. Shifting the row by one for
    // every digit we get the answer to part 2 in as little computation as possible
    let coeffs = (0..(INPUT_LEN * REPETITION - OFFSET) as u32)
        .map(|i| binom_mod10(ITERATIONS - 1 + i, i))
        .collect::<Vec<_>>();

    let blocks = OFFSET / INPUT_LEN;

    // For every digit
    for k in 0..8 {
        // Start iterating through the coefficients, this will automatically advance between the
        // different blocks we use it and keep track or where we are
        let mut coeffs = coeffs.iter().copied();

        let mut d = 0;

        // Consider the part of the input that is not repeated fully
        d += unsigned_signal
            .iter()
            .skip(OFFSET - blocks * INPUT_LEN + k)
            .zip(coeffs.by_ref())
            .map(|(a, b)| a * b)
            .sum::<u32>();

        // Then consider rest of the input which is repeated in toto
        for _ in blocks + 1..REPETITION {
            d += unsigned_signal
                .iter()
                .zip(coeffs.by_ref())
                .map(|(a, b)| a * b)
                .sum::<u32>();
        }

        // Limit the digit to be, well, a digit and show it
        write!(lock, "{}", d % 10).unwrap();
    }
}

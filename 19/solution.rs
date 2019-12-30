const PART1_SIDE: u64 = 50;
const PART2_SIDE: u64 = 100;

// No actual intcode can be seen here because I chose to reverse-engineer the code and figure out
// what it was doing
fn present_in_row(y: u64) -> u64 {
    5 * (y / 11)
        + match y % 11 {
            0 => 1,
            n => (n + 1) / 2,
        }
}

fn x_offset(y: u64) -> u64 {
    17 * (y / 11)
        + match y % 11 {
            0 => 0,
            1 => 2,
            2 => 4,
            3 => 5,
            4 => 7,
            5 => 8,
            6 => 10,
            7 => 11,
            8 => 13,
            9 => 14,
            10 => 16,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
}

fn in_beam(x: u64, y: u64) -> bool {
    let off = x_offset(y);
    x >= off && (x - off) < present_in_row(y)
}

fn main() {
    println!(
        "{}",
        (0..PART1_SIDE)
            .map(|y| (0..PART1_SIDE).filter(|&x| in_beam(x, y)).count())
            .sum::<usize>()
    );

    println!(
        "{}",
        (0..)
            .skip_while(|&top| present_in_row(top) < PART2_SIDE)
            .find_map(|top| {
                // restrict the topleft corner to be at the end of the top row
                let left = x_offset(top) + present_in_row(top) - PART2_SIDE;

                // if the other three corners are in the beam, the whole square is
                if in_beam(left, top + PART2_SIDE - 1)
                    && in_beam(left, top + PART2_SIDE - 1)
                    && in_beam(left + PART2_SIDE - 1, top + PART2_SIDE - 1)
                {
                    Some(10000 * left + top)
                } else {
                    None
                }
            })
            .unwrap()
    );
}

use std::collections::HashSet;

use intcode::Interpreter;

const MIN_ROUTINE_LEN: usize = 6;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Move {
    RotateRight,
    RotateLeft,
    Forward(usize),

    RoutineA,
    RoutineB,
    RoutineC,
}

impl Move {
    fn is_routine(&self) -> bool {
        *self == Move::RoutineA || *self == Move::RoutineB || *self == Move::RoutineC
    }
}

fn add_move(path: &mut Vec<Move>, steps: usize) {
    if steps == 0 {
        return;
    }

    if let Some(Move::Forward(n)) = path.last_mut() {
        *n += steps;
    } else {
        path.push(Move::Forward(steps));
    }
}

fn apply_routine(path: &[Move], routine: &[Move], routine_move: Move) -> Vec<Move> {
    debug_assert!(!routine.is_empty());

    let mut rest = Vec::with_capacity(path.len());

    let mut k = 0;
    while k < path.len() {
        if path[k..].starts_with(routine) {
            rest.push(routine_move);
            k += routine.len();
        } else {
            rest.push(path[k]);
            k += 1;
        }
    }

    rest
}

fn path_input(path: &[Move]) -> Vec<i64> {
    let mut result = Vec::with_capacity(path_len(path) + 1);

    for (i, step) in path.iter().enumerate() {
        match step {
            Move::RotateLeft => result.push(b'L' as i64),
            Move::RotateRight => result.push(b'R' as i64),
            Move::Forward(n) => result.extend(n.to_string().bytes().map(|c| c as i64)),
            Move::RoutineA => result.push(b'A' as i64),
            Move::RoutineB => result.push(b'B' as i64),
            Move::RoutineC => result.push(b'C' as i64),
        }

        result.push(if i == path.len() - 1 {
            b'\n' as i64
        } else {
            b',' as i64
        });
    }

    result
}

fn digits(n: usize) -> usize {
    debug_assert!(n < 100);
    if n < 10 {
        1
    } else {
        2
    }
}

fn path_len(path: &[Move]) -> usize {
    path.iter()
        .map(|&step| {
            if let Move::Forward(n) = step {
                digits(n)
            } else {
                1
            }
        })
        .sum::<usize>()
        + (path.len() - 1)
}

fn main() {
    let mut interpreter = Interpreter::from_input(include_str!("input.txt"));
    interpreter.memory.extend_from_slice(&[0; 2000]);
    interpreter.memory[0] = 2;

    interpreter.run();

    let mut scaffolds: HashSet<(i64, i64)> = HashSet::new();

    let mut position = (0, 0);

    {
        let mut y = 0;
        let mut x = 0;

        for c in interpreter.output.drain(..) {
            let c = c as u8;

            if c == b'\n' {
                y += 1;
                x = 0;
                continue;
            } else if c == b'^' {
                position = (x, y);
            } else if c == b'#' {
                scaffolds.insert((x, y));
            } else if c != b'.' {
                break;
            }

            x += 1;
        }
    }

    println!(
        "{}",
        scaffolds
            .iter()
            .filter(|&&(x, y)| {
                scaffolds.contains(&(x + 1, y))
                    && scaffolds.contains(&(x - 1, y))
                    && scaffolds.contains(&(x, y + 1))
                    && scaffolds.contains(&(x, y - 1))
            })
            .map(|(x, y)| x * y)
            .sum::<i64>()
    );

    // [UP, RIGHT, DOWN, LEFT]
    let mut orientation = 0u8;

    // how many steps we've taken since the last rotation
    let mut steps = 0usize;

    // how many rotations we've taken since the last step
    let mut rotations = 0u8;

    let mut path: Vec<Move> = Vec::new();

    while !scaffolds.is_empty() {
        let (x, y) = position;

        let tentative_position = match orientation {
            0 => (x, y - 1),
            1 => (x + 1, y),
            2 => (x, y + 1),
            3 => (x - 1, y),
            _ => unreachable!(),
        };

        if scaffolds.remove(&tentative_position) || rotations == 4 {
            if rotations == 1 {
                path.push(Move::RotateRight);
            } else if rotations == 3 {
                path.push(Move::RotateLeft);
            }

            position = tentative_position;
            steps += 1;
            rotations = 0;
        } else {
            orientation = (orientation + 1) % 4;

            add_move(&mut path, steps);

            steps = 0;
            rotations += 1;
        }
    }
    add_move(&mut path, steps);

    // Restriction: The main sequence must begin with A,B

    for a_end in MIN_ROUTINE_LEN..path.len() {
        let a = &path[..a_end];
        let path = apply_routine(&path, a, Move::RoutineA);

        for b_end in MIN_ROUTINE_LEN..path.len() {
            let b = &path[1..b_end];
            let path = apply_routine(&path, b, Move::RoutineB);

            // C must be everything that remains between routines
            let mut it = path.iter();
            let c_start = it.by_ref().position(|m| !m.is_routine()).unwrap();
            let c_end = (c_start + 1) + it.position(|m| m.is_routine()).unwrap_or(0);

            let c = &path[c_start..c_end];

            let path = apply_routine(&path, c, Move::RoutineC);

            if path_len(&path) < 20 {
                interpreter.input.extend(path_input(&path));
                interpreter.input.extend(path_input(&a));
                interpreter.input.extend(path_input(&b));
                interpreter.input.extend(path_input(&c));
                interpreter.input.push_back(b'n' as i64);
                interpreter.input.push_back(b'\n' as i64);

                interpreter.run();

                println!("{}", interpreter.output.back().unwrap());

                return;
            }
        }
    }
}

use std::{
    cmp::{max, min},
    collections::HashSet,
};

include!("../intcode.rs");

const DIRECTIONS: [(i64, i64); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn main() {
    let mut program = load_program(include_str!("input.txt"));
    program.extend_from_slice(&[Cell::Value(0); 500]);

    let run = |whites: &mut HashSet<(i64, i64)>, painted: &mut HashSet<(i64, i64)>| {
        let mut interpreter = Interpreter::new(program.clone());

        let mut x = 0i64;
        let mut y = 0i64;
        let mut d = 0;

        loop {
            interpreter
                .input
                .push_back(if whites.contains(&(x, y)) { 1 } else { 0 });

            interpreter.run();

            if interpreter.done {
                break;
            }

            let color = interpreter.output.pop_front().unwrap();
            let direction = interpreter.output.pop_front().unwrap();

            match color {
                0 => {
                    whites.remove(&(x, y));
                }
                1 => {
                    whites.insert((x, y));
                }
                _ => unreachable!(),
            }
            painted.insert((x, y));

            if direction == 0 {
                d += 2;
            }

            let (dx, dy) = DIRECTIONS[d & 3];
            x += dx;
            y += dy;

            d += 1;
        }
    };

    let mut whites = HashSet::new();
    let mut painted = HashSet::new();

    run(&mut whites, &mut painted);

    println!("{}", painted.len());

    whites.clear();
    whites.insert((0, 0));
    painted.clear();

    run(&mut whites, &mut painted);

    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    for &(x, y) in &whites {
        min_x = min(x, min_x);
        max_x = max(x, max_x);
        min_y = min(y, min_y);
        max_y = max(y, max_y);
    }

    // the for loops constitute a 90° CCW rotation
    // (x, y) -> (-y, x)
    for x in min_x..=max_x {
        for y in (min_y..=max_y).rev() {
            print!("{}", if whites.contains(&(x, y)) { '█' } else { ' ' });
        }

        println!();
    }
}

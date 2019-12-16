use std::{
    cmp::{max, min},
    collections::HashMap,
};

include!("../intcode.rs");

const DIRECTIONS: [(i64, i64); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn main() {
    let mut program = load_program(include_str!("input.txt"));
    program.extend_from_slice(&[Cell::Value(0); 500]);

    let run = |colors: &mut HashMap<(i64, i64), i64>| {
        let mut interpreter = Interpreter::new(program.clone());

        let mut x = 0i64;
        let mut y = 0i64;
        let mut d = 0;

        loop {
            interpreter
                .input
                .push_back(colors.get(&(x, y)).copied().unwrap_or(0));

            interpreter.run();

            if interpreter.done {
                break;
            }

            let color = interpreter.output.pop_front().unwrap();
            let direction = interpreter.output.pop_front().unwrap();

            colors.insert((x, y), color);

            if direction == 0 {
                d += 2;
            }

            let (dx, dy) = DIRECTIONS[d & 3];
            x += dx;
            y += dy;

            d += 1;
        }
    };

    let mut colors = HashMap::new();

    run(&mut colors);

    println!("{}", colors.len());

    colors.clear();
    colors.insert((0, 0), 1);

    run(&mut colors);

    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    for (x, y) in colors.keys() {
        min_x = min(*x, min_x);
        max_x = max(*x, max_x);
        min_y = min(*y, min_y);
        max_y = max(*y, max_y);
    }

    // the for loops constitute a 90° CCW rotation
    // (x, y) -> (-y, x)
    for x in min_x..=max_x {
        for y in (min_y..=max_y).rev() {
            print!(
                "{}",
                if colors.get(&(x, y)) == Some(&1) {
                    '█'
                } else {
                    ' '
                }
            );
        }

        println!();
    }
}

use std::{
    cmp::{max, min},
    collections::HashMap,
    io::{self, Write},
};

use intcode::*;

const DIRECTIONS: [(i8, i8); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn run(program: Vec<Cell>, colors: &mut HashMap<(i8, i8), bool>) {
    let mut interpreter = Interpreter::new(program);

    let mut x = 0i8;
    let mut y = 0i8;
    let mut d = 0usize;

    loop {
        interpreter
            .input
            .push_back(colors.get(&(x, y)).copied().unwrap_or(false) as i64);

        interpreter.run();

        if interpreter.done {
            break;
        }

        let color = interpreter.output.pop_front().unwrap();
        let direction = interpreter.output.pop_front().unwrap();

        colors.insert((x, y), color != 0);

        if direction == 0 {
            d += 2;
        }

        let (dx, dy) = DIRECTIONS[d & 3];
        x += dx;
        y += dy;

        d += 1;
    }
}

fn main() {
    let mut program = load_program(include_str!("input.txt"));
    program.extend_from_slice(&[Cell::Value(0); 500]);

    let mut colors = HashMap::new();

    run(program.clone(), &mut colors);

    println!("{}", colors.len());

    colors.clear();
    colors.insert((0, 0), true);

    run(program, &mut colors);

    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    for (x, y) in colors.keys().copied() {
        min_x = min(x, min_x);
        max_x = max(x, max_x);
        min_y = min(y, min_y);
        max_y = max(y, max_y);
    }

    // the for loops constitute a 90° CCW rotation
    // (x, y) -> (-y, x)
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for x in min_x..=max_x {
        for y in (min_y..=max_y).rev() {
            write!(
                handle,
                "{}",
                if colors.get(&(x, y)).copied().unwrap_or(false) {
                    '█'
                } else {
                    ' '
                }
            )
            .unwrap();
        }

        writeln!(handle).unwrap();
    }
}

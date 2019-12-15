use std::collections::HashMap;

type Screen = HashMap<(i64, i64), i64>;

const BLOCK: i64 = 2;
const PADDLE: i64 = 3;
const BALL: i64 = 4;

include!("../intcode.rs");

fn main() {
    let mut program = load_program(include_str!("input.txt"));
    program.extend_from_slice(&[Cell::Value(0); 100]);

    let initial_screen = {
        let mut interpreter = Interpreter::new(program.clone());
        interpreter.run();

        interpreter
            .output
            .as_slices()
            .0
            .chunks(3)
            .map(|chunk| ((chunk[0], chunk[1]), chunk[2]))
            .collect::<Screen>()
    };

    // paddle x coordinate
    let mut p_x = 0;

    // ball x coordinate
    let mut b_x = None;
    let mut b_x0 = 0;

    let mut part1 = 0;

    for ((x, y), tile) in initial_screen {
        match tile {
            BLOCK => part1 += 1,
            PADDLE => p_x = x,
            BALL => b_x0 = x,
            _ => {}
        }
    }

    println!("{}", part1);

    program[0] = Cell::Value(2);

    let mut interpreter = Interpreter::new(program);

    let mut screen: Screen = HashMap::new();

    let mut score = 0;

    while !interpreter.done {
        let joystick;

        if let Some(b_x) = b_x {
            let b_fx: i64 = b_x + (b_x - b_x0) - 1;

            joystick = (b_fx - p_x).signum();
            p_x += joystick;

            b_x0 = b_x;
        } else {
            joystick = 0;
        }

        interpreter.input.push_back(joystick);

        interpreter.run();

        while !interpreter.output.is_empty() {
            let x = interpreter.output.pop_front().unwrap();
            let y = interpreter.output.pop_front().unwrap();
            let tile = interpreter.output.pop_front().unwrap();

            if x == -1 && y == 0 {
                score = tile;
            } else if tile == BALL {
                b_x = Some(x);
            }

            screen.insert((x, y), tile);
        }
    }

    println!("{}", score);
}

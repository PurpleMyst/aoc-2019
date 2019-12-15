const BLOCK: i64 = 2;
const BALL: i64 = 4;

include!("../intcode.rs");

fn main() {
    let mut program = load_program(include_str!("input.txt"));
    program.extend_from_slice(&[Cell::Value(0); 14]);

    let part1 = {
        let mut interpreter = Interpreter::new(program.clone());
        interpreter.run();

        interpreter
            .output
            .into_iter()
            .skip(2)
            .step_by(3)
            .filter(|&tile| tile == BLOCK)
            .count()
    };
    println!("{}", part1);

    // paddle x coordinate
    let mut p_x = 21;

    // ball x coordinate
    let mut b_x = p_x - 2;

    program[0] = Cell::Value(2);

    let mut interpreter = Interpreter::new(program);

    let mut score = 0;

    while !interpreter.done {
        interpreter.input.push_back(if b_x < p_x {
            p_x -= 1;
            -1
        } else if b_x != p_x {
            p_x += 1;
            1
        } else {
            0
        });

        interpreter.run();

        while !interpreter.output.is_empty() {
            let x = interpreter.output.pop_front().unwrap();
            let y = interpreter.output.pop_front().unwrap();
            let tile = interpreter.output.pop_front().unwrap();

            if x == -1 && y == 0 {
                score = tile;
            } else if tile == BALL {
                b_x = x;
            }
        }
    }

    println!("{}", score);
}

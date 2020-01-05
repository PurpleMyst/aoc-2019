use intcode::Interpreter;

const BLOCK: i64 = 2;
const BALL: i64 = 4;

fn main() {
    let mut interpreter = Interpreter::from_input(include_str!("input.txt"));
    interpreter.memory.extend_from_slice(&[0; 14]);

    let part1 = {
        let mut interpreter = interpreter.clone();
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

    interpreter.memory[0] = 2;

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

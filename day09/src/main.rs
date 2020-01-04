use intcode::*;

fn main() {
    let mut program = load_program(include_str!("input.txt"));
    program.extend_from_slice(&[Cell::Value(0); 1080 - 973]);

    let mut interpreter = Interpreter::new(program.clone());
    interpreter.input.push_back(1);
    interpreter.run();

    println!("{:?}", interpreter.output[0]);

    let mut interpreter = Interpreter::new(program);
    interpreter.input.push_back(2);
    interpreter.run();

    println!("{:?}", interpreter.output[0]);
}

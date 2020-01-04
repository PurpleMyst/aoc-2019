use intcode::*;

fn main() {
    let mut program = load_program(include_str!("input.txt"));
    program.extend_from_slice(&[Cell::Value(0); 2000]);

    let part1 = {
        let mut interpreter = Interpreter::new(program.clone());
        interpreter
            .input
            .extend(from_ascii(include_str!("springscript1.txt")));
        interpreter.run();
        *interpreter.output.back().unwrap()
    };

    let part2 = {
        let mut interpreter = Interpreter::new(program);
        interpreter
            .input
            .extend(from_ascii(include_str!("springscript2.txt")));
        interpreter.run();
        *interpreter.output.back().unwrap()
    };

    println!("{}", part1);
    println!("{}", part2);
}

include!("../intcode.rs");

fn main() {
    let mut program = load_program(include_str!("input.txt"));
    program.extend_from_slice(&[Cell::Value(0); 2000]);

    let part1 = {
        let mut interpreter = Interpreter::new(program.clone());
        interpreter
            .input
            .extend(include_str!("springscript1.txt").bytes().map(|c| c as i64));
        interpreter.run();
        *interpreter.output.back().unwrap()
    };

    let part2 = {
        let mut interpreter = Interpreter::new(program);
        interpreter
            .input
            .extend(include_str!("springscript2.txt").bytes().map(|c| c as i64));
        interpreter.run();
        *interpreter.output.back().unwrap()
    };

    println!("{}", part1);
    println!("{}", part2);
}

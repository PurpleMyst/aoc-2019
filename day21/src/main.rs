use intcode::Interpreter;

fn main() {
    let mut interpreter = Interpreter::from_input(include_str!("input.txt"));
    interpreter.memory.extend_from_slice(&[0; 2000]);

    let part1 = {
        let mut interpreter = interpreter.clone();
        interpreter.input_from_ascii(include_str!("springscript1.txt"));
        interpreter.run();
        *interpreter.output.back().unwrap()
    };

    let part2 = {
        let mut interpreter = interpreter.clone();
        interpreter.input_from_ascii(include_str!("springscript2.txt"));
        interpreter.run();
        *interpreter.output.back().unwrap()
    };

    println!("{}", part1);
    println!("{}", part2);
}

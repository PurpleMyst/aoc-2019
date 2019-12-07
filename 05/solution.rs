include!("../intcode.rs");

fn main() {
    let program = load_program(include_str!("input.txt"));

    let run = |input| {
        let mut interpreter = Interpreter::new(program.clone());
        interpreter.input.push_back(input);
        interpreter.run();
        interpreter.output.pop_back().unwrap()
    };

    // Part 1
    println!("{}", run(1));

    // Part 2
    println!("{}", run(5));
}

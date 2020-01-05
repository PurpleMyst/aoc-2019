use intcode::Interpreter;

fn main() {
    let interpreter = Interpreter::from_input(include_str!("input.txt"));

    let run = |input| {
        let mut interpreter = interpreter.clone();
        interpreter.input.push_back(input);
        interpreter.run();
        interpreter.output.pop_back().unwrap()
    };

    // Part 1
    println!("{}", run(1));

    // Part 2
    println!("{}", run(5));
}

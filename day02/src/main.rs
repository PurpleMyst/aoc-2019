use intcode::Interpreter;

const TARGET: i64 = 19690720;

fn main() {
    let interpreter = Interpreter::from_input(include_str!("input.txt"));

    let run = |noun, verb| {
        let mut interpreter = interpreter.clone();
        interpreter.memory[1] = noun;
        interpreter.memory[2] = verb;
        interpreter.run();
        i64::from(interpreter.memory[0])
    };

    println!("{}", run(12, 2));

    let mut options = Vec::with_capacity(100 * 100);
    for noun in 0..100 {
        for verb in 0..100 {
            options.push((noun, verb));
        }
    }

    println!(
        "{}",
        options
            .binary_search_by_key(&TARGET, |(noun, verb)| run(*noun, *verb))
            .unwrap()
    );
}

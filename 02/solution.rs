include!("../intcode.rs");

const TARGET: isize = 19690720;

fn main() {
    let program = load_program(include_str!("input.txt"));

    let run = |noun, verb| {
        let mut interpreter = Interpreter::new(program.clone());
        interpreter.program[1] = noun;
        interpreter.program[2] = verb;
        interpreter.run();
        isize::from(interpreter.program[0])
    };

    println!("{}", run(Cell::from(12), Cell::from(2)));

    let mut options = Vec::with_capacity(100 * 100);
    for noun in (0..100).map(Cell::from) {
        for verb in (0..100).map(Cell::from) {
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

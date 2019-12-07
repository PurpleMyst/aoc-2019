include!("../intcode.rs");

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

    let (noun, verb) = (0..100)
        .flat_map(|noun| (0..100).map(move |verb| (Cell::from(noun), Cell::from(verb))))
        .find(|(noun, verb)| run(*noun, *verb) == 19690720)
        .unwrap();

    println!("{}", isize::from(noun) * 100 + isize::from(verb));
}

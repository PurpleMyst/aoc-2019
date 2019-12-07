mod intcode {
    include!("../intcode.rs");
}

fn main() {
    let program = include_str!("input.txt")
        .trim()
        .split(',')
        .map(|n| <From<usize>>::from(n.parse().unwrap()))
        .collect::<Vec<intcode::Cell>>;

    println!("{}", interpet_changing(&program, 12, 2));

    let (noun, verb) = (0..100)
        .flat_map(|noun| (0..100).map(move |verb| (intcode::Cell::from(noun), intcode::Cell::from(verb))))
        .find(|(noun, verb)| {
            let mut program = program.clone();
            program[1] = noun;
            program[2] = verb;
            intcode::interpret(&mut program) == 19690720
        })
        .unwrap();

    println!("{}", noun * 100 + verb);
}

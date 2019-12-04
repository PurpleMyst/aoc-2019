const ADD: usize = 1;
const MUL: usize = 2;
const HALT: usize = 99;

fn interpret(program: &mut [usize]) {
    let mut pc = 0;

    loop {
        match program[pc] {
            ADD => program[program[pc + 3]] = program[program[pc + 1]] + program[program[pc + 2]],

            MUL => program[program[pc + 3]] = program[program[pc + 1]] * program[program[pc + 2]],

            HALT => break,

            _ => unreachable!(),
        }

        pc += 4;
    }
}

fn interpet_changing(program: &[usize], noun: usize, verb: usize) -> usize {
    let mut program = program.to_owned();
    program[1] = noun;
    program[2] = verb;
    interpret(&mut program);
    program[0]
}

fn main() {
    let program = include_str!("input.txt")
        .trim()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<_>>();

    println!("{}", interpet_changing(&program, 12, 2));

    let (noun, verb) = (0..100)
        .flat_map(|noun| (0..100).map(move |verb| (noun, verb)))
        .find(|(noun, verb)| interpet_changing(&program, *noun, *verb) == 19690720)
        .unwrap();
    println!("{}", noun * 100 + verb);
}

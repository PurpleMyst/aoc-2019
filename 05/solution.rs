mod intcode {
    include!("../intcode.rs");
}

fn main() {
    use intcode::*;

    let mut program = include_str!("input.txt")
        .trim()
        .split(",")
        .map(|n| <Cell as From<isize>>::from(n.parse().unwrap()))
        .collect::<Vec<_>>();

    // Part 1
    println!(
        "{:?}",
        interpret(program.clone().as_mut_slice(), 1).last().unwrap()
    );

    // Part 2
    println!("{:?}", interpret(program.as_mut_slice(), 5).last().unwrap());
}

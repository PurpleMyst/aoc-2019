mod first {
    pub fn fuel(mass: isize) -> isize {
        mass / 3 - 2
    }
}

mod second {
    pub fn fuel(mass: isize) -> isize {
        use super::first;
        std::iter::successors(Some(first::fuel(mass)), |&f| Some(first::fuel(f)))
            .take_while(|&n| n > 0)
            .sum()
    }
}

fn main() {
    let modules = include_str!("input.txt")
        .lines()
        .map(|line| line.parse().unwrap());

    println!("{}", modules.clone().map(first::fuel).sum::<isize>());

    println!("{}", modules.map(second::fuel).sum::<isize>());
}

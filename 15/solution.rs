use std::collections::HashMap;

mod intcode {
    include!("../intcode.rs");
}
use intcode::*;

struct Solver {
    interpreter: Interpreter,
    spaces: HashMap<(i64, i64), i64>,
    distances: HashMap<(i64, i64), usize>,
    oxygen: Option<(i64, i64)>,
}

impl Solver {
    fn solve(&mut self, distance: usize, x: i64, y: i64) {
        // Create an array of candidates
        let candidates = [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)];

        // For each candidate
        for (i, (x, y)) in candidates.iter().copied().enumerate() {
            // If we have not visited yet
            if self.spaces.contains_key(&(x, y)) {
                continue;
            }

            // Tell the robot to visit it
            let dir = i as i64 + 1;
            self.interpreter.input.push_front(dir);
            self.interpreter.run();

            // Get the robot's information about it and store it
            let space = self.interpreter.output.pop_front().unwrap();
            self.spaces.insert((x, y), space);
            self.distances.insert((x, y), distance + 1);

            // If we couldn't move there, just go on to the next candidate
            if space == 0 {
                continue;
            }

            // If it was the oxygen system, save its position
            if space == 2 {
                self.oxygen = Some((x, y));
            }

            self.solve(distance + 1, x, y);

            // After we've explored that point fully, tell the bot to move back to the original
            // position and continue with the next candidate
            self.interpreter.input.push_front(match dir {
                1 => 2,
                2 => 1,
                3 => 4,
                4 => 3,
                _ => unreachable!(),
            });
            self.interpreter.run();
            assert_ne!(self.interpreter.output.pop_front(), Some(0));
        }
    }
}

fn fills_in((x, y): (i64, i64), spaces: &mut HashMap<(i64, i64), i64>) -> usize {
    let mut result = 0;

    let candidates = [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)];
    for (x, y) in candidates.iter().copied() {
        if let Some(1) = spaces.remove(&(x, y)) {
            result = std::cmp::max(result, 1 + fills_in((x, y), spaces))
        }
    }

    result
}

fn main() {
    let program = load_program(include_str!("input.txt"));
    let interpreter = Interpreter::new(program);

    let mut spaces = HashMap::new();
    spaces.insert((0, 0), 1);

    let mut solver = Solver {
        spaces,
        interpreter,
        distances: Default::default(),
        oxygen: None,
    };
    solver.solve(0, 0, 0);

    println!(
        "{}",
        solver
            .distances
            .get(solver.oxygen.as_ref().unwrap())
            .unwrap()
    );

    println!("{}", fills_in(solver.oxygen.unwrap(), &mut solver.spaces));
}

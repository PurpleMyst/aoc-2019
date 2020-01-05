use std::collections::HashMap;

use intcode::Interpreter;

fn opposite(dir: i64) -> i64 {
    match dir {
        1 => 2,
        2 => 1,
        3 => 4,
        4 => 3,
        _ => unreachable!(),
    }
}

struct Solver {
    interpreter: Interpreter,
    distances: HashMap<(i64, i64), usize>,
    oxygen: Option<(i64, i64)>,
    look_for: i64,
}

impl Solver {
    fn solve(&mut self, distance: usize, x: i64, y: i64) -> Result<(), ()> {
        // Create an array of candidates
        let candidates = [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)];

        // For each candidate unexplored spot
        for (i, (x, y)) in candidates.iter().copied().enumerate() {
            // If we have not explored it yet
            if self.distances.contains_key(&(x, y)) {
                continue;
            }

            // Tell the robot to visit it
            let dir = i as i64 + 1;
            self.interpreter.input.push_front(dir);
            self.interpreter.run();

            // Get the robot's information about it
            let space = self.interpreter.output.pop_front().unwrap();
            self.distances.insert((x, y), distance + 1);

            // If we couldn't move there, move on
            if space == 0 {
                continue;
            }
            // Otherwise, if it was the oxygen system, save its position
            if space == self.look_for {
                self.oxygen = Some((x, y));
                return Err(());
            }

            // And keep on exploring from there
            self.solve(distance + 1, x, y)?;

            // After we've explored that point fully, tell the bot to move back to the original
            // position and continue with the next candidate
            self.interpreter.input.push_front(opposite(dir));
            self.interpreter.run();
            assert_ne!(self.interpreter.output.pop_front(), Some(0));
        }

        Ok(())
    }
}

fn main() {
    let interpreter = Interpreter::from_input(include_str!("input.txt"));

    // Initialize our solver and set it to look for the oxygen
    let mut solver = Solver {
        interpreter,
        distances: HashMap::new(),
        oxygen: None,
        look_for: 2,
    };
    solver.solve(0, 0, 0).unwrap_err();

    // Print out the solution to part1 by getting the distance from the start to the oxygen
    let oxygen = solver.oxygen.take().unwrap();
    println!("{}", solver.distances.get(&oxygen).unwrap());

    // Now, reset the information, make the drone aimless and make it wander around until we've
    // explored everything
    solver.distances.clear();
    solver.look_for = -1;
    solver.solve(0, oxygen.0, oxygen.1).unwrap();

    // Get the solution to part2 by figuring out the farthest point from the start
    println!("{}", solver.distances.values().max().unwrap() - 1);
}

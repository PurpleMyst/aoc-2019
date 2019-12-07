use std::{collections::HashMap, iter};

#[derive(Default)]
struct Solver {
    // for part 1
    open: HashMap<&'static str, Vec<&'static str>>,
    ranks: HashMap<&'static str, usize>,
    total: usize,

    // for part 2
    parents: HashMap<&'static str, &'static str>,
}

impl Solver {
    fn add_orbit(&mut self, orbited: &'static str, orbiter: &'static str) {
        self.parents.insert(orbiter, orbited);

        if let Some(rank) = self.ranks.get(orbited) {
            let rank = rank + 1;
            self.ranks.insert(orbiter, rank);
            self.total += rank;

            if let Some(pending) = self.open.remove(orbiter) {
                pending
                    .into_iter()
                    .for_each(|orbiter2| self.add_orbit(orbiter, orbiter2));
            }
        } else {
            self.open.entry(orbited).or_default().push(orbiter);
        }
    }
}

fn main() {
    let mut solver = Solver::default();
    solver.ranks.insert("COM", 0);

    include_str!("input.txt")
        .lines()
        .map(|s| {
            let mut splat = s.split(')');
            (splat.next().unwrap(), splat.next().unwrap())
        })
        .for_each(|(orbited, orbiter)| solver.add_orbit(orbited, orbiter));

    println!("{}", solver.total);

    let parents = |start| iter::successors(Some(start), |&cur| solver.parents.get(cur).copied());

    let chain = parents("YOU")
        .enumerate()
        .map(|(v, k)| (k, v))
        .collect::<HashMap<_, _>>();

    let (dist, common) = parents("SAN")
        .enumerate()
        .find(|(_, node)| chain.contains_key(node))
        .unwrap();

    println!("{}", (dist - 1) + (chain.get(common).unwrap() - 1));
}

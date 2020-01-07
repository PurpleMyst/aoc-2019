use std::{cell::Cell, cmp::Ordering, collections::HashMap};

const PART2_ORE: usize = 1_000_000_000_000;

type Ingredient = (usize, &'static str);
type Reaction = (&'static str, (usize, Vec<Ingredient>));

fn parse_ingredient(product_per_recipe: &'static str) -> Ingredient {
    let mut splat = product_per_recipe.splitn(2, ' ');
    let n = splat.next().unwrap().parse().unwrap();
    let product = splat.next().unwrap();
    (n, product)
}

fn parse_reaction(reaction: &'static str) -> Reaction {
    let mut sides = reaction.splitn(2, " => ");

    let lhs = sides
        .next()
        .unwrap()
        .split(", ")
        .map(parse_ingredient)
        .collect::<Vec<_>>();

    let (n, rhs) = parse_ingredient(sides.next().unwrap());

    (rhs, (n, lhs))
}

struct Solver<'a> {
    recipes: &'a HashMap<&'static str, (usize, Vec<Ingredient>)>,
    had: HashMap<&'static str, Cell<usize>>,
    ore_cost: Cell<usize>,
}

impl<'a> Solver<'a> {
    fn new(recipes: &'a HashMap<&'static str, (usize, Vec<Ingredient>)>) -> Self {
        Self {
            had: recipes.keys().map(|key| (*key, Cell::default())).collect(),
            ore_cost: Cell::default(),
            recipes,
        }
    }

    fn make(&self, product: &'static str, needed: usize) {
        // If we're making ORE, we just need to keep track of how much we need
        if product == "ORE" {
            self.ore_cost.set(self.ore_cost.get() + needed);
            return;
        }

        // We don't need to make what we already have
        let product_had = self.had.get(product).unwrap();

        // If needed - product_had underflows we already have enough product
        if let Some(needed) = needed.checked_sub(product_had.get()) {
            // Figure out how many the recipe makes
            let (product_per_recipe, ingredients) = self.recipes.get(product).unwrap();

            // How many times do we need to run the recipe?
            let recipes_made = (needed + (product_per_recipe - 1)) / product_per_recipe;

            for &(needed, ingredient) in ingredients {
                // Make it ...
                self.make(ingredient, needed * recipes_made);

                // ... and use it
                if ingredient != "ORE" {
                    let ingredient_had = self.had.get(ingredient).unwrap();
                    ingredient_had.set(ingredient_had.get() - needed * recipes_made);
                }
            }

            // And produce the result
            product_had.set(product_had.get() + recipes_made * product_per_recipe)
        }
    }
}

fn main() {
    let recipes = include_str!("input.txt")
        .lines()
        .map(parse_reaction)
        .collect::<HashMap<_, _>>();

    let solver = Solver::new(&recipes);
    solver.make("FUEL", 1);
    let part1 = solver.ore_cost.get();
    println!("{}", part1);

    // Part one represents the maximum cost for 1 FUEL because we started from scratch. Due to
    // this, we can calculate a lower bound on the solution by considering how much we could make
    // assuming we started from scratch every time
    let mut l = PART2_ORE / part1;
    let mut r = PART2_ORE;

    while l <= r {
        let fuel = (l + r) / 2;

        let solver = Solver::new(&recipes);
        solver.make("FUEL", fuel);

        match solver.ore_cost.get().cmp(&PART2_ORE) {
            Ordering::Less => l = fuel + 1,
            Ordering::Greater => r = fuel - 1,
            Ordering::Equal => break,
        }
    }

    println!("{}", (l + r) / 2);
}

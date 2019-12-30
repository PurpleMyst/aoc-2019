use std::{cell::Cell, collections::HashMap};

type Ingredient<'a> = (usize, &'a str);
type Reaction<'a> = (&'a str, (usize, Vec<Ingredient<'a>>));

fn parse_ingredient(product_per_recipe: &str) -> Ingredient {
    let mut splat = product_per_recipe.splitn(2, ' ');
    let n = splat.next().unwrap().parse().unwrap();
    let product = splat.next().unwrap();
    (n, product)
}

fn parse_reaction(reaction: &str) -> Reaction {
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
    recipes: &'a HashMap<&'a str, (usize, Vec<Ingredient<'a>>)>,
    had: HashMap<&'a str, Cell<usize>>,
    ore_cost: Cell<usize>,
}

impl<'a> Solver<'a> {
    fn new(recipes: &'a HashMap<&'a str, (usize, Vec<Ingredient<'a>>)>) -> Self {
        Self {
            had: recipes.keys().map(|key| (*key, Cell::default())).collect(),
            ore_cost: Cell::default(),
            recipes,
        }
    }

    fn make(&self, product: &'a str, needed: usize) {
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
    println!("{}", solver.ore_cost.get());

    const HAVE_ORE: usize = 1_000_000_000_000;

    let mut l = 0;
    let mut r = HAVE_ORE - 1;

    while l <= r {
        let fuel = (l + r) / 2;

        let solver = Solver::new(&recipes);
        solver.make("FUEL", fuel);

        let ore = solver.ore_cost.get();

        if ore < HAVE_ORE {
            l = fuel + 1;
        } else if ore > HAVE_ORE {
            r = fuel - 1;
        }
    }

    println!("{}", (l + r) / 2);
}

use std::ops::BitOr;

const GRID_SIDE: i8 = 5;

const LAYERS: usize = 201;

type Point2D = (i8, i8);

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Layer(u32);

fn mask((x, y): Point2D) -> u32 {
    1 << (y * GRID_SIDE + x)
}

impl Layer {
    fn set(&mut self, point: Point2D) {
        self.0 |= mask(point);
    }

    fn get(self, point: Point2D) -> bool {
        self.0 & mask(point) != 0
    }

    fn alive_in_mask(self, mask: u32) -> u32 {
        (self.0 & mask).count_ones()
    }

    fn alive_total(self) -> u32 {
        self.0.count_ones()
    }

    fn biodiversity(self) -> u32 {
        self.0
    }
}

fn four_neighborhood_mask((x, y): Point2D) -> u32 {
    [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
        .iter()
        .copied()
        .filter(|&(x, y)| x >= 0 && x < GRID_SIDE && y >= 0 && y < GRID_SIDE)
        .map(mask)
        .fold(0, BitOr::bitor)
}

fn plutonian_masks((x, y): Point2D) -> (u32, u32, u32) {
    let mut up_mask = 0;
    let mut cur_mask = 0;
    let mut down_mask = 0;

    for &(dx, dy) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let x = x + dx;
        let y = y + dy;

        if x == GRID_SIDE / 2 && y == GRID_SIDE / 2 {
            let mask = match (dx, dy) {
                (1, 0) => (0..GRID_SIDE)
                    .map(|y| (0, y))
                    .map(mask)
                    .fold(0, BitOr::bitor),
                (-1, 0) => (0..GRID_SIDE)
                    .map(|y| (GRID_SIDE - 1, y))
                    .map(mask)
                    .fold(0, BitOr::bitor),
                (0, 1) => (0..GRID_SIDE)
                    .map(|x| (x, 0))
                    .map(mask)
                    .fold(0, BitOr::bitor),
                (0, -1) => (0..GRID_SIDE)
                    .map(|x| (x, GRID_SIDE - 1))
                    .map(mask)
                    .fold(0, BitOr::bitor),
                _ => unsafe { std::hint::unreachable_unchecked() },
            };

            down_mask |= mask;
        } else if x < 0 || y < 0 || x >= GRID_SIDE || y >= GRID_SIDE {
            up_mask |= mask((dx + GRID_SIDE / 2, dy + GRID_SIDE / 2));
        } else {
            cur_mask |= mask((x, y));
        }
    }

    (up_mask, cur_mask, down_mask)
}

fn part1_step(layer: Layer) -> Layer {
    let mut next = Layer(0);
    for y in 0..GRID_SIDE {
        for x in 0..GRID_SIDE {
            let neighbors = layer.alive_in_mask(four_neighborhood_mask((x, y)));

            if neighbors == 1 || (neighbors == 2 && !layer.get((x, y))) {
                next.set((x, y));
            }
        }
    }
    next
}

fn part2_step(layers: [Layer; LAYERS]) -> [Layer; LAYERS] {
    let mut next = [Layer(0); LAYERS];
    for z in 0..LAYERS {
        // Ignore patches of three which are empty: these will not change
        if (z == 0 || layers[z - 1] == Layer(0))
            && layers[z] == Layer(0)
            && (z == LAYERS - 1 || layers[z + 1] == Layer(0))
        {
            continue;
        }

        for y in 0..GRID_SIDE {
            for x in 0..GRID_SIDE {
                if x == GRID_SIDE / 2 && y == GRID_SIDE / 2 {
                    continue;
                }

                let (up_mask, cur_mask, down_mask) = plutonian_masks((x, y));

                let up_neighbors = if z == 0 {
                    0
                } else {
                    layers[z - 1].alive_in_mask(up_mask)
                };

                let down_neighbors = if z == LAYERS - 1 {
                    0
                } else {
                    layers[z + 1].alive_in_mask(down_mask)
                };

                let neighbors = up_neighbors + layers[z].alive_in_mask(cur_mask) + down_neighbors;

                if neighbors == 1 || (neighbors == 2 && !layers[z].get((x, y))) {
                    next[z].set((x, y));
                }
            }
        }
    }

    next
}

fn main() {
    let mut layer = Layer(0);

    include_str!("input.txt")
        .lines()
        .enumerate()
        .for_each(|(y, row)| {
            row.bytes().enumerate().for_each(|(x, col)| {
                if col == b'#' {
                    layer.set((x as i8, y as i8));
                }
            })
        });

    {
        let mut layer = layer;
        let mut seen = std::collections::HashSet::new();
        while seen.insert(layer) {
            layer = part1_step(layer);
        }
        println!("{}", layer.biodiversity());
    }

    let mut layers = [Layer(0); LAYERS];
    layers[LAYERS / 2] = layer;

    for _ in 0..200 {
        layers = part2_step(layers);
    }
    println!(
        "{}",
        layers.iter().map(|layer| layer.alive_total()).sum::<u32>()
    );
}

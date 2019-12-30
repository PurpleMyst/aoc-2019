use std::{
    collections::{BinaryHeap, HashMap},
    f64::consts::{FRAC_PI_2, PI},
    hash::{Hash, Hasher},
    hint::unreachable_unchecked,
    iter::FromIterator,
};

#[derive(Clone, Copy, Debug, PartialEq)]
struct OrderedFloat(f64);

impl Eq for OrderedFloat {}

impl PartialOrd for OrderedFloat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.0.partial_cmp(&other.0) {
            Some(ordering) => ordering,
            None => unsafe { unreachable_unchecked() },
        }
    }
}

impl Hash for OrderedFloat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state)
    }
}

/// Calculate the clockwise angle from the positive y axis from one point to another
fn angle_between((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> f64 {
    let mut theta = FRAC_PI_2 - f64::atan2((y2 - y1) as f64, (x2 - x1) as f64);

    if theta < 0.0 {
        theta += 2.0 * PI;
    }

    theta
}

fn main() {
    let asteroids = include_str!("input.txt")
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.bytes().enumerate().filter_map(move |(x, c)| {
                if c == b'#' {
                    // invert the y coordinate to use atan2 correctly
                    Some((x as i64, -(y as i64)))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();

    // Find the asteroid with the most unique angles around it. We only care about the last one of
    // the group because part 1's answer is always greater than 200
    let angles = asteroids
        .iter()
        .copied()
        .map(|station| {
            asteroids
                .iter()
                .copied()
                .map(|asteroid| (OrderedFloat(angle_between(station, asteroid)), asteroid))
                .collect::<HashMap<OrderedFloat, (i64, i64)>>()
        })
        .max_by_key(|map| map.len())
        .unwrap();

    // The answer to part 1 is the length of the map
    println!("{}", angles.len());

    // Sort the map by angle to get everything in destruction order
    let map = BinaryHeap::from_iter(angles);

    // The 200th element to be pulled out of the heap is our part 2 answer
    let (_, (x, y)) = map.into_sorted_vec()[199];
    println!("{}", 100 * x - y);
}

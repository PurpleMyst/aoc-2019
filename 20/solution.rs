use std::{
    cmp::Reverse,
    collections::{hash_map::Entry, BinaryHeap, HashMap, HashSet},
};

const UPPER_EDGE: u8 = 2;
const LOWER_EDGE: u8 = 122;
const LEFT_EDGE: u8 = 2;
const RIGHT_EDGE: u8 = 126;

type Point2D = (u8, u8);
type Point3D = (u8, u8, i16);

macro_rules! neighbors {
    ($position:expr) => {{
        let (x, y) = $position;
        [
            (Some(x + 1), Some(y)),
            (x.checked_sub(1), Some(y)),
            (Some(x), Some(y + 1)),
            (Some(x), y.checked_sub(1)),
        ]
        .iter()
        .copied()
        .filter_map(|(x, y)| Some((x?, y?)))
    }};
}

fn heuristic(d: i16) -> i16 {
    (d + 1) * (125 / 2)
}

fn pathfind(
    empty: &HashSet<Point2D>,
    portals: &HashMap<Point2D, Point2D>,
    start: Point2D,
    end: Point2D,
    new_depth: impl Fn(Point2D) -> i16,
) -> i16 {
    let mut open = BinaryHeap::new();
    open.push((Reverse(heuristic(0)), 0, (start.0, start.1, 0)));

    let mut gs = HashMap::new();
    gs.insert((start.0, start.1, 0), 0);

    // (X, Y) -> ((X, Y, ΔZ), ΔG)
    let graph: HashMap<Point2D, Vec<(Point3D, i16)>> = empty
        .iter()
        .map(|&(x, y)| {
            (
                (x, y),
                neighbors!((x, y))
                    .filter(|&(x, y)| empty.contains(&(x, y)))
                    .map(|(x, y)| {
                        portals
                            .get(&(x, y))
                            .copied()
                            .map(|(x2, y2)| ((x2, y2, new_depth((x, y))), 2))
                            .unwrap_or(((x, y, 0), 1))
                    })
                    .collect(),
            )
        })
        .collect();

    while let Some((_, g, (x, y, d))) = open.pop() {
        if d == 0 && (x, y) == end {
            return g;
        }

        open.extend(
            graph[&(x, y)]
                .iter()
                .copied()
                .filter_map(|((x, y, dd), dg)| {
                    let d = d + dd;
                    let g = g + dg;

                    if d < 0 {
                        return None;
                    }

                    match gs.entry((x, y, d)) {
                        Entry::Occupied(mut entry) => {
                            if g < *entry.get() {
                                entry.insert(g);
                            } else {
                                return None;
                            }
                        }

                        Entry::Vacant(entry) => {
                            entry.insert(g);
                        }
                    }

                    let f = g + heuristic(d);
                    Some((Reverse(f), g, (x, y, d)))
                }),
        )
    }

    unreachable!("could not find path");
}

fn main() {
    let map = include_str!("input.txt")
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.bytes()
                .enumerate()
                .map(move |(x, c)| ((x as u8, y as u8), c))
        })
        .filter(|&(_, c)| c != b' ' && c != b'#')
        .collect::<HashMap<Point2D, u8>>();

    let mut portals = HashMap::with_capacity(map.len());
    let mut unpaired = HashMap::with_capacity(map.len());

    map.iter()
        .filter(|(_, c)| c.is_ascii_uppercase())
        .filter_map(|(&(x, y), &c1)| {
            let mut c2 = None;
            let mut trampoline = None;
            for neighbor in neighbors!((x, y)) {
                if trampoline.is_some() && c2.is_some() {
                    break;
                }

                match map.get(&neighbor) {
                    Some(&c) if c.is_ascii_uppercase() => c2 = Some(c),
                    Some(b'.') => trampoline = Some(neighbor),
                    _ => {}
                }
            }

            if let (Some(trampoline), Some(c2)) = (trampoline, c2) {
                let name = if c1 < c2 { (c1, c2) } else { (c2, c1) };
                Some((trampoline, name))
            } else {
                None
            }
        })
        .for_each(|(pos1, portal)| match unpaired.entry(portal) {
            Entry::Occupied(entry) => {
                let pos2 = entry.remove();
                portals.reserve(2);
                portals.insert(pos1, pos2);
                portals.insert(pos2, pos1);
            }

            Entry::Vacant(entry) => {
                entry.insert(pos1);
            }
        });

    let empty = map
        .into_iter()
        .filter_map(|(pos, c)| if c == b'.' { Some(pos) } else { None })
        .collect::<HashSet<Point2D>>();

    let start = unpaired.remove(&(b'A', b'A')).unwrap();
    let end = unpaired.remove(&(b'Z', b'Z')).unwrap();

    println!("{}", pathfind(&empty, &portals, start, end, |_| 0));

    println!(
        "{}",
        pathfind(&empty, &portals, start, end, |(x, y)| {
            let outer = x == LEFT_EDGE || x == RIGHT_EDGE || y == UPPER_EDGE || y == LOWER_EDGE;

            if outer {
                -1
            } else {
                1
            }
        })
    );
}

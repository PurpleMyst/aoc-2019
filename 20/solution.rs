use std::{
    cmp::Reverse,
    collections::{hash_map::Entry, BinaryHeap, HashMap, HashSet},
    iter::once,
};

type Point = (u8, u8);

const UPPER_EDGE: u8 = 2;
const LOWER_EDGE: u8 = 122;
const LEFT_EDGE: u8 = 2;
const RIGHT_EDGE: u8 = 126;

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

// (X1, Y1) -> [(DISTANCE, DEPTH_CHANGE, (X2, Y2))]
type Graph = HashMap<Point, Vec<(i16, i16, Point)>>;

fn make_graph(
    empty: &HashSet<Point>,
    labels: &HashMap<Point, Point>,
    start: Point,
    end: Point,
    depth_change: impl Fn(Point) -> i16,
) -> Graph {
    let mut stack = Vec::with_capacity(labels.len() / 2);
    let mut visited = HashSet::with_capacity(empty.len());

    labels
        .keys()
        .copied()
        .chain(once(start))
        .chain(once(end))
        .map(|label| {
            let mut neighbors = Vec::with_capacity(labels.len() / 2);

            stack.clear();
            stack.push((label, 0));
            visited.clear();

            while let Some((current, distance)) = stack.pop() {
                if !visited.insert(current) {
                    continue;
                }

                if current == end {
                    neighbors.push((distance, 0, current));
                    continue;
                }

                if let Some(&destination) = labels.get(&current) {
                    neighbors.push((distance + 1, depth_change(current), destination));
                }

                stack.extend(
                    neighbors!(current)
                        .filter(|neighbor| empty.contains(neighbor))
                        .map(|neighbor| (neighbor, distance + 1)),
                )
            }

            (label, neighbors)
        })
        .collect()
}

fn heuristic(d: i16) -> i16 {
    (RIGHT_EDGE - LEFT_EDGE) as i16 * d
}

fn pathfind(graph: Graph, start: Point, end: Point) -> i16 {
    let mut open = BinaryHeap::new();
    open.push((Reverse(heuristic(0)), 0, (start.0, start.1, 0)));

    let mut gs = HashMap::new();
    gs.insert((start.0, start.1, 0), 0);

    while let Some((_, g, (x, y, d))) = open.pop() {
        if d == 0 && (x, y) == end {
            return g;
        }

        open.extend(
            graph[&(x, y)]
                .iter()
                .copied()
                .filter_map(|(dg, dd, (x, y))| {
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

fn is_outer((x, y): Point) -> bool {
    x == LEFT_EDGE || x == RIGHT_EDGE || y == UPPER_EDGE || y == LOWER_EDGE
}

fn depth_change(label: Point) -> i16 {
    if is_outer(label) {
        -1
    } else {
        1
    }
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
        .collect::<HashMap<Point, u8>>();

    let mut labels = HashMap::new();
    let mut unpaired = HashMap::new();

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
        .for_each(|(pos1, label)| match unpaired.entry(label) {
            Entry::Occupied(entry) => {
                let pos2 = entry.remove();
                labels.insert(pos1, pos2);
                labels.insert(pos2, pos1);
            }

            Entry::Vacant(entry) => {
                entry.insert(pos1);
            }
        });

    let empty = map
        .into_iter()
        .filter_map(|(pos, c)| if c == b'.' { Some(pos) } else { None })
        .collect::<HashSet<Point>>();

    let start = unpaired.remove(&(b'A', b'A')).unwrap();
    let end = unpaired.remove(&(b'Z', b'Z')).unwrap();

    println!(
        "{}",
        pathfind(make_graph(&empty, &labels, start, end, |_| 0), start, end)
    );

    println!(
        "{}",
        pathfind(
            make_graph(&empty, &labels, start, end, depth_change),
            start,
            end
        )
    );
}

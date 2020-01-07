use std::{cmp::Ordering, collections::*};

const EMPTY: u8 = b'.';
const ENTRANCE: u8 = b'@';

type Point = (usize, usize);

/// A Graph is a mapping from a Point to the states you can reach from it
type Graph = HashMap<Point, Vec<State>>;

mod keys;
use keys::{Keys, KEYS};

fn abs_diff(a: usize, b: usize) -> usize {
    if let Some(n) = a.checked_sub(b) {
        n
    } else {
        b - a
    }
}

fn manhattan((x1, y1): Point, (x2, y2): Point) -> usize {
    abs_diff(x1, x2) + abs_diff(y1, y2)
}

#[derive(Hash, Clone, Copy, Debug)]
struct State {
    position: Point,
    distance: usize,
    keys_gained: Keys,
    keys_needed: Keys,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for State {}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// States are ordered so that the BinaryHeap in `solve` considers the best solutions first.
// A state is considered "greater" than another if it has more keys gained or less steps taken.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.keys_gained.count().cmp(&other.keys_gained.count()))
            .then(self.distance.cmp(&other.distance).reverse())
    }
}

/// Calculate the shortest path picking up all the keys
fn solve(initial_position: Point, initial_keys: Keys, graph: HashMap<Point, Vec<State>>) -> usize {
    // What states do we need to visit next? we utilize a BinaryHeap so that we find new solutions
    // as fast as possible so that we can prune a lot of states with the `< solution` check below.
    let mut states: BinaryHeap<State> = BinaryHeap::new();

    // What states have we already considered? There's no need to consider them again.
    let mut seen: HashSet<State> = HashSet::new();

    states.push(State {
        position: initial_position,
        distance: 0,
        keys_gained: initial_keys,
        keys_needed: Keys::none(),
    });

    let mut solution = std::usize::MAX;

    while let Some(state) = states.pop() {
        if !seen.insert(state) {
            continue;
        }

        if state.keys_gained.count() == 26 {
            solution = state.distance;
            continue;
        }

        states.extend(
            graph
                .get(&state.position)
                .unwrap()
                .iter()
                .copied()
                .filter_map(|next_state| {
                    // Consider only states which we can reach, which give us new keys and which
                    // aren't trivially worse than the best solution so far
                    if state.keys_gained.has_all(next_state.keys_needed)
                        && !state.keys_gained.has_all(next_state.keys_gained)
                        && state.distance + next_state.distance < solution
                    {
                        Some(State {
                            position: next_state.position,
                            distance: state.distance + next_state.distance,
                            keys_gained: state.keys_gained.add_all(next_state.keys_gained),
                            keys_needed: Keys::none(),
                        })
                    } else {
                        None
                    }
                }),
        );
    }

    solution
}

fn neighbors((x, y): Point) -> [Point; 4] {
    [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
}

/// Create the Graph for a given quadrant
fn build_graph(mut quadrant: HashMap<Point, u8>, entrance: Point) -> Graph {
    // Remove the entrance from consideration
    quadrant.remove(&entrance);

    // Map points to what point comes before it in a breadth-first search
    let mut parents = HashMap::with_capacity(quadrant.len());

    // Pre-allocate the graph with enough capacity for all the keys and the entrance.
    let mut graph: Graph = HashMap::with_capacity(KEYS as usize + 1);

    graph.insert(entrance, Vec::with_capacity(KEYS as usize));

    // Run a breadth-first search
    let mut q = Vec::with_capacity(quadrant.len());
    q.push(State {
        position: entrance,
        distance: 0,
        keys_gained: Keys::none(),
        keys_needed: Keys::none(),
    });
    while let Some(state) = q.pop() {
        for &position in &neighbors(state.position) {
            // We remove from the quadrant so that we only consider points once
            if let Some(symbol) = quadrant.remove(&position) {
                let mut next_state = State {
                    distance: state.distance + 1,
                    position,
                    ..state
                };

                if symbol.is_ascii_lowercase() {
                    // If this position is a key, the next state will gain that key
                    next_state.keys_gained = next_state.keys_gained.add(symbol);
                    // Add the key to the edges from the start
                    graph.get_mut(&entrance).unwrap().push(next_state);
                } else if symbol.is_ascii_uppercase() {
                    // If this position is a door, to get to the next state we will to own its
                    // corresponding key
                    next_state.keys_needed = next_state.keys_needed.add(symbol.to_ascii_lowercase())
                }

                parents.insert(next_state.position, state.position);
                q.push(next_state);
            }
        }
    }

    // Calculate parents from a point
    let parents =
        |point| std::iter::successors(Some(point), |p| parents.get(p).copied()).enumerate();

    let all_keys = graph.get(&entrance).unwrap().clone();

    // For all pairs of keys ...
    for (i, state1) in all_keys.iter().copied().enumerate() {
        for state2 in all_keys[i + 1..].iter().copied() {
            let mut parents1 = parents(state1.position);
            let parents2 = parents(state2.position);

            let parents2 = parents2.map(|(d, p)| (p, d)).collect::<HashMap<_, _>>();

            // The distance between two keys in the same quadrant is the sum of the distances from
            // each key to their nearest common ancestor
            let distance = parents1
                .find_map(|(d, p)| Some(d + parents2.get(&p)?))
                .unwrap();

            let mut add_to_graph = |state1: State, state2: State| {
                graph.entry(state1.position).or_default().push(State {
                    position: state2.position,
                    distance,
                    keys_needed: state1.keys_needed.add_all(state2.keys_needed),
                    keys_gained: state1.keys_gained.add_all(state2.keys_gained),
                })
            };

            add_to_graph(state1, state2);
            add_to_graph(state2, state1);
        }
    }

    // Avoid considering states which are redundant:
    // If from node A when can reach nodes B and C and reaching node C requires going through node
    // B but not through any doors (that you haven't already gone through while reaching B), then
    // there's no reason to consider the state (A -> B) because key C must be picked up anyways
    for edges in graph.values_mut() {
        // Group states by keys required to reach them
        let mut groups: HashMap<Keys, Vec<State>> = Default::default();
        edges
            .iter()
            .for_each(|&state| groups.entry(state.keys_needed).or_default().push(state));

        let mut new_edges: HashSet<State> = HashSet::new();
        for (_, mut group) in groups {
            // For all nodes in the current group
            for _ in 0..group.len() {
                let node1 = group.remove(0);
                new_edges.insert(node1);

                // If any of the nodes not considered yet render this one redundant (as explained
                // above), remove it from the new edges
                for &node2 in &group {
                    if node2.keys_gained.has_all(node1.keys_gained) {
                        new_edges.remove(&node1);
                        break;
                    }
                }
            }
        }

        *edges = new_edges.into_iter().collect();
    }

    graph
}

fn main() {
    let mut input = include_str!("input.txt")
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.bytes()
                .enumerate()
                .filter(|&(_, c)| c != b'#')
                .map(move |(x, c)| ((x, y), c))
        });

    // The problem input, but not any of the example inputs, can be divided into four quadrants
    // which are perfect mazes. This allows us to exploit a few properties of perfect mazes, mainly
    // that they are equivalent to a tree. This allows us to calculate distances between keys
    // without requiring anything but BFS and simple arithmetic.

    // Find the initial position, remembering all that we skip over
    let mut initial_position = (0, 0);
    let mut unconsidered = Vec::new();
    for ((x, y), c) in input.by_ref() {
        unconsidered.push(((x, y), c));

        if c == ENTRANCE {
            initial_position = (x, y);
        }
    }

    // Now divide everything else into quadrants
    let mut quadrants = vec![HashMap::new(); 4];
    let mut quadrant_initial_keys = vec![Keys::all(); 4];
    let mut initial_keys = Keys::all();
    for ((x, y), c) in unconsidered.into_iter().chain(input) {
        let idx = match (x.cmp(&initial_position.0), y.cmp(&initial_position.1)) {
            (Ordering::Equal, _) | (_, Ordering::Equal) => continue,
            (Ordering::Less, Ordering::Less) => 0,
            (Ordering::Greater, Ordering::Less) => 1,
            (Ordering::Greater, Ordering::Greater) => 2,
            (Ordering::Less, Ordering::Greater) => 3,
        };
        let quadrant = &mut quadrants[idx];

        if c == EMPTY {
            quadrant.insert((x, y), EMPTY);
        } else if c == ENTRANCE {
            quadrant.insert((x, y), EMPTY);
            initial_position = (x, y);
        } else if c.is_ascii_lowercase() {
            quadrant.insert((x, y), c);
            initial_keys = initial_keys.remove(c);
            quadrant_initial_keys[idx] = quadrant_initial_keys[idx].remove(c);
        } else if c.is_ascii_uppercase() {
            quadrant.insert((x, y), c);
        }
    }

    let entrances = [
        (initial_position.0 - 1, initial_position.1 - 1),
        (initial_position.0 + 1, initial_position.1 - 1),
        (initial_position.0 + 1, initial_position.1 + 1),
        (initial_position.0 - 1, initial_position.1 + 1),
    ];

    let mut graphs = quadrants
        .into_iter()
        .zip(entrances.iter().copied())
        .map(|(quadrant, entrance)| (build_graph(quadrant, entrance), entrance))
        .collect::<Vec<_>>();

    let part2 = graphs
        .clone()
        .into_iter()
        .zip(quadrant_initial_keys.into_iter())
        .map(|((graph, entrance), initial_keys)| solve(entrance, initial_keys, graph))
        .sum::<usize>();

    let mut graph: Graph = HashMap::with_capacity(KEYS as usize + 1);

    graph.insert(initial_position, Vec::with_capacity(KEYS as usize));

    // For all pairs of quadrants
    for _ in 0..graphs.len() {
        let (mut graph1, entrance1) = graphs.remove(0);

        graph.get_mut(&initial_position).unwrap().extend(
            graph1.get(&entrance1).unwrap().iter().map(|&state| State {
                distance: state.distance + manhattan(initial_position, entrance1),
                ..state
            }),
        );

        for (graph2, entrance2) in graphs.iter() {
            // For each pair of keys in the two quadrants
            for &state1 in graph1.get(&entrance1).unwrap().iter() {
                for &state2 in graph2.get(&entrance2).unwrap().iter() {
                    // The distance between two keys in two separate quadrants is the distance from
                    // the first key to its entrance, plus the distance from the first key's
                    // entrance to the second key's entrance, plus the distance from the second
                    // key's entrance to the second key.
                    let distance =
                        state1.distance + manhattan(entrance1, *entrance2) + state2.distance;

                    let mut add_to_graph = |state1: State, state2: State| {
                        graph.entry(state1.position).or_default().push(State {
                            position: state2.position,
                            distance,
                            keys_needed: state1.keys_needed.add_all(state2.keys_needed),
                            keys_gained: state1.keys_gained.add_all(state2.keys_gained),
                        })
                    };

                    add_to_graph(state1, state2);
                    add_to_graph(state2, state1);
                }
            }
        }

        // Add all
        graph1.remove(&entrance1);
        graph1
            .into_iter()
            .for_each(|(pos, states)| graph.entry(pos).or_default().extend(states));
    }

    println!("{}", solve(initial_position, initial_keys, graph));
    println!("{}", part2);
}

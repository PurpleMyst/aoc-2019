use std::{cmp::Ordering, collections::*};

const EMPTY: u8 = b'.';
const ENTRANCE: u8 = b'@';

const KEYS: u32 = 26;

type Point = (usize, usize);

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash, Default)]
struct Keys(u32);

fn key_to_idx(key: u8) -> u8 {
    debug_assert!(key.is_ascii_lowercase());
    key - b'a'
}

impl Keys {
    fn all() -> Self {
        Self((1 << KEYS) - 1)
    }

    fn none() -> Self {
        Self(0)
    }

    fn add(self, key: u8) -> Self {
        Self(self.0 | (1 << key_to_idx(key)))
    }

    fn add_all(self, keys: Keys) -> Self {
        Self(self.0 | keys.0)
    }

    fn remove(self, key: u8) -> Self {
        Self(self.0 ^ (self.0 & (1 << key_to_idx(key))))
    }

    fn has_all(self, required: Self) -> bool {
        self.0 & required.0 == required.0
    }

    fn count(self) -> u32 {
        self.0.count_ones()
    }
}

macro_rules! make_ord {
    ($struct:ident => |$self:ident, $other:ident| $cmp:expr) => {
        impl PartialEq for $struct {
            fn eq(&self, other: &Self) -> bool {
                self.cmp(other) == Ordering::Equal
            }
        }

        impl Eq for $struct {}

        impl PartialOrd for $struct {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $struct {
            fn cmp(&self, $other: &Self) -> Ordering {
                let $self = self;
                $cmp
            }
        }
    };
}

#[derive(Clone, Copy, Debug, Hash)]
struct State {
    /// Where does this state leave you?
    position: Point,

    /// How long is the path from the source node to the final position?
    distance: usize,

    /// What keys does this state require?
    keys_used: Keys,

    /// What keys does this state pick up along the way?
    keys_gained: Keys,
}

make_ord!(State => |this, other| {
    this.keys_gained.count().cmp(&other.keys_gained.count())
        .then(this.distance.cmp(&other.distance).reverse())
});

fn distance_to_keys(map: &HashMap<Point, u8>, source: Point) -> HashMap<u8, State> {
    let mut result: HashMap<u8, State> = HashMap::with_capacity(KEYS as usize);

    let mut states = BinaryHeap::with_capacity(map.len());

    states.push(State {
        position: source,
        keys_used: Keys::none(),
        keys_gained: Keys::none(),
        distance: 0,
    });

    let mut visited = HashSet::with_capacity(map.len());
    visited.insert(source);

    while let Some(state) = states.pop() {
        let (x, y) = state.position;
        let candidates = [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];

        for &position in &candidates {
            if !visited.insert(position) {
                continue;
            }

            let mut next_state = State {
                position,
                distance: state.distance + 1,
                ..state
            };

            match map.get(&position).copied() {
                Some(EMPTY) => {}

                // if we step on a key just be fine with it alright
                Some(c @ b'a'..=b'z') => {
                    next_state.keys_gained = next_state.keys_gained.add(c);
                    result.insert(c, next_state);
                }

                // if we step on a door this path requires the key
                Some(c @ b'A'..=b'Z') => {
                    next_state.keys_used = next_state.keys_used.add(c.to_ascii_lowercase());
                }

                Some(..) => unsafe { std::hint::unreachable_unchecked() },

                None => continue,
            }

            states.push(next_state);
        }
    }

    result
}

fn solve(map: &HashMap<Point, u8>, initial_position: Point, initial_keys: Keys) -> usize {
    let mut states: BinaryHeap<State> = BinaryHeap::new();

    let mut seen: HashSet<State> = HashSet::new();

    let mut cache: HashMap<Point, HashMap<u8, State>> = HashMap::with_capacity(KEYS as usize);

    states.push(State {
        position: initial_position,
        distance: 0,
        keys_gained: initial_keys,
        keys_used: Keys::none(),
    });

    let mut solution = std::usize::MAX;

    while let Some(state) = states.pop() {
        if !seen.insert(state) {
            continue;
        }

        if state.keys_gained.count() == KEYS {
            solution = state.distance;
            continue;
        }

        states.extend(
            (cache.entry(state.position))
                .or_insert_with(|| distance_to_keys(&map, state.position))
                .values()
                // Only consider states that we can get to
                .filter(|&next_state| state.keys_gained.has_all(next_state.keys_used))
                // Only consider states that make us gain keys
                .filter(|&next_state| !state.keys_gained.has_all(next_state.keys_gained))
                // Only consider states that could potentially be a winner
                .filter(|&next_state| (state.distance + next_state.distance) < solution)
                // Combine the next state and the current state
                .map(|next_state| State {
                    position: next_state.position,
                    distance: state.distance + next_state.distance,
                    keys_used: Keys::none(),
                    keys_gained: state.keys_gained.add_all(next_state.keys_gained),
                }),
        );
    }

    solution
}

fn main() {
    let input = include_str!("input.txt")
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.bytes()
                .enumerate()
                .filter(|&(_, c)| c != b'#')
                .map(move |(x, c)| ((x, y), c))
        });

    let mut map: HashMap<Point, u8> = Default::default();

    let mut initial_keys = Keys::all();

    let mut initial_position = (0, 0);

    for ((x, y), c) in input {
        if c == EMPTY {
            map.insert((x, y), EMPTY);
        } else if c == ENTRANCE {
            map.insert((x, y), EMPTY);
            initial_position = (x, y);
        } else if c.is_ascii_lowercase() {
            map.insert((x, y), c);
            initial_keys = initial_keys.remove(c);
        } else if c.is_ascii_uppercase() {
            map.insert((x, y), c);
        }
    }

    println!("{:?}", solve(&map, initial_position, initial_keys));

    let mut quadrants = vec![(HashMap::new(), Keys::all()); 4];

    for ((x, y), c) in map {
        let i = match (x.cmp(&initial_position.0), y.cmp(&initial_position.1)) {
            (Ordering::Equal, _) | (_, Ordering::Equal) => continue,
            (Ordering::Less, Ordering::Less) => 0,
            (Ordering::Greater, Ordering::Less) => 1,
            (Ordering::Greater, Ordering::Greater) => 2,
            (Ordering::Less, Ordering::Greater) => 3,
        };

        let (map, initial_keys) = &mut quadrants[i];
        map.insert((x, y), c);
        if c.is_ascii_lowercase() {
            *initial_keys = initial_keys.remove(c);
        }
    }

    println!(
        "{}",
        quadrants
            .into_iter()
            .enumerate()
            .map(|(i, (map, initial_keys))| std::thread::spawn(move || {
                let initial_position = match i {
                    0 => (initial_position.0 - 1, initial_position.1 - 1),
                    1 => (initial_position.0 + 1, initial_position.1 - 1),
                    2 => (initial_position.0 + 1, initial_position.1 + 1),
                    3 => (initial_position.0 - 1, initial_position.1 + 1),
                    _ => unsafe { std::hint::unreachable_unchecked() },
                };

                solve(&map, initial_position, initial_keys)
            }))
            .collect::<Vec<_>>()
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .sum::<usize>()
    );
}

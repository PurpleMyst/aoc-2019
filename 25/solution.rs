use std::collections::HashSet;

include!("../intcode.rs");

const NL: i64 = b'\n' as i64;

// if we're in the Security Checkpoint

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Room {
    name: String,
    doors: Vec<Door>,
    items: Vec<String>,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Door {
    North,
    South,
    East,
    West,
}

impl Door {
    fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}

impl std::str::FromStr for Door {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "north" => Ok(Self::North),
            "south" => Ok(Self::South),
            "east" => Ok(Self::East),
            "west" => Ok(Self::West),
            _ => Err(format!("Invalid door {:?}", s)),
        }
    }
}

impl Into<&'static str> for Door {
    fn into(self) -> &'static str {
        match self {
            Self::North => "north",
            Self::South => "south",
            Self::East => "east",
            Self::West => "west",
        }
    }
}

fn line(output: &mut VecDeque<i64>) -> String {
    let mut buf = String::new();

    loop {
        match output.pop_front() {
            Some(NL) | None => break,
            Some(c) => buf.push(c as u8 as char),
        }
    }

    buf
}

fn skip_double_line(output: &mut VecDeque<i64>) {
    let mut last = output.pop_front().unwrap();

    loop {
        match output.pop_front() {
            Some(NL) if last == NL => break,
            Some(c) => last = c,
            None => unreachable!(),
        }
    }
}

fn list<T>(output: &mut VecDeque<i64>) -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut result = Vec::new();
    loop {
        let mut item = line(output);

        if item.is_empty() {
            break;
        }

        // Remove the leading " -"
        item.drain(..2);

        result.push(item.parse().unwrap());
    }
    result
}

impl Room {
    fn parse(output: &mut VecDeque<i64>) -> Self {
        // Skip leading newlines
        while output.front() == Some(&NL) {
            output.pop_front();
        }

        // The next line is the room's name.
        let mut name = line(output);

        // Remove the leading/trailing ==
        name.drain(..3);
        name.drain(name.len() - 3..);

        // Everything until the next double newline is the room's description. Just ignore it.
        skip_double_line(output);

        // Then the next line is "Doors here lead:";
        let doors_header = line(output);
        debug_assert_eq!(doors_header, "Doors here lead:");

        // Until the next empty line, every line is a door
        let doors = list(output);

        // If the items are present they're also a list
        let items = if output.front() == Some(&(b'I' as i64)) {
            // skip "Items here:"
            line(output);

            list(output)
        } else {
            vec![]
        };

        // skip "Command?"
        if output.front() == Some(&(b'C' as i64)) {
            line(output);
        }

        Room { name, doors, items }
    }
}

fn is_safe(item: &str) -> bool {
    match item {
        "photons" | "escape pod" | "infinite loop" | "molten lava" | "giant electromagnet" => false,
        _ => true,
    }
}

struct Droid {
    visited_rooms: HashSet<String>,
    interpreter: Interpreter,
    items: HashSet<String>,
}

impl Droid {
    fn new(interpreter: Interpreter) -> Self {
        Self {
            interpreter,
            visited_rooms: HashSet::new(),
            items: HashSet::new(),
        }
    }

    fn take(&mut self, item: String) {
        self.interpreter.input.extend(from_ascii("take "));
        self.interpreter.input.extend(from_ascii(&item));
        self.interpreter.input.push_back(NL);
        self.interpreter.run();
        self.interpreter.output.clear();

        self.items.insert(item);
    }

    fn drop(&mut self, item: &str) -> String {
        self.interpreter.input.extend(from_ascii("drop "));
        self.interpreter.input.extend(from_ascii(item));
        self.interpreter.input.push_back(NL);
        self.interpreter.run();
        self.interpreter.output.clear();

        self.items
            .take(item)
            .expect("dropped an item we didn't have")
    }

    fn enter(&mut self, door: Door) {
        self.interpreter.input.extend(from_ascii(door.into()));
        self.interpreter.input.push_back(NL);
        self.interpreter.run();
    }

    fn explore(&mut self, Room { name, items, doors }: Room) {
        // Don't explore from the Security Checkpoint because of the Pressure-Sensitive Floor
        if name == "Security Checkpoint" {
            return;
        }

        // Don't visit what we've already visited
        if !self.visited_rooms.insert(name) {
            return;
        }

        // Take every item we can
        items
            .iter()
            .filter(|item| is_safe(item))
            .cloned()
            .for_each(|item| self.take(item));

        // Visit every possible room
        for door in doors.iter().copied() {
            // Tell the game to visit the room
            self.enter(door);

            // Parse the information about the room
            let new_room = Room::parse(&mut self.interpreter.output);

            // Keep searching from that room
            self.explore(new_room);

            // Afterwards, return to the original room
            self.enter(door.opposite());

            // Ignore the output
            self.interpreter.output.clear();
        }
    }
}

#[derive(Debug, Copy, PartialEq, Eq, Clone)]
enum Weight {
    TooHeavy,
    TooLight,
    JustRight,
}

fn analyse(output: &VecDeque<i64>) -> Weight {
    let s = to_ascii(output.iter().copied()).collect::<String>();

    if s.contains("heavier") {
        Weight::TooLight
    } else if s.contains("lighter") {
        Weight::TooHeavy
    } else {
        Weight::JustRight
    }
}

fn main() {
    let mut program = load_program(include_str!("input.txt"));
    program.extend_from_slice(&[Cell::Value(0); 1000]);
    let mut interpreter = Interpreter::new(program);

    interpreter.run();
    let room = Room::parse(&mut interpreter.output);

    let mut droid = Droid::new(interpreter);

    // Explore all possible rooms and pick up everything
    droid.explore(room);

    // Go to the security room. Going west now will make us enter the pressure-sensitive floor
    droid.enter(Door::South);
    droid.enter(Door::East);
    droid.enter(Door::South);

    // Remove all the useless output
    droid.interpreter.output.clear();

    // Drop everything the droid is currently holding
    let mut floor: Vec<_> = droid
        .items
        .clone()
        .iter()
        .map(|item| droid.drop(item))
        .collect();

    // For every item on the floor
    for i in (0..floor.len()).rev() {
        // Get the item from the floor
        let item = floor.remove(i);
        droid.take(item.clone());

        // Enter the pressure-sensitive room
        droid.enter(Door::West);
        let weight = analyse(&droid.interpreter.output);
        droid.interpreter.output.clear();

        // Drop the item
        droid.drop(&item);

        // If the item doesn't make us too heavy even without anything else, add it back to the
        // floor vector. Otherwise just pretend it doesn't exist.
        if weight == Weight::TooLight {
            floor.push(item);
        }
    }

    // Pick everything back up
    floor.into_iter().for_each(|item| droid.take(item));

    let mut optional = Vec::with_capacity(droid.items.len());
    for item in droid.items.clone() {
        // Drop just one item at a time
        let item_copy = droid.drop(&item);

        // Analyse our weight with everything but that item
        droid.enter(Door::West);
        let weight = analyse(&droid.interpreter.output);
        droid.interpreter.output.clear();

        // Pick it back up
        droid.take(item_copy);

        // If we're not too light without the item, the item is not necessarily required to have
        // the correct weight
        if weight != Weight::TooLight {
            optional.push(item);
        }
    }

    // Now, armed with our knowledge of what we mustn't drop ...
    let mut floor = Vec::with_capacity(optional.len());

    // Check every possible item set, dropping some of the optional items every time
    for i in 0u8..(1 << optional.len()) {
        for j in 0..optional.len() {
            if i & (1 << j) != 0 {
                floor.push(droid.drop(&optional[j]));
            }
        }

        droid.enter(Door::West);
        let weight = analyse(&droid.interpreter.output);

        if weight == Weight::JustRight {
            to_ascii(droid.interpreter.output.into_iter())
                .skip_while(|&c| c != '"')
                .for_each(|c| print!("{}", c));

            return;
        }

        droid.interpreter.output.clear();

        // Otherwise, pick everything back up and try again
        floor.drain(..).for_each(|item| droid.take(item));
    }
}

use std::collections::{HashSet, VecDeque};

use intcode::*;

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
    items: Vec<String>,
}

impl Droid {
    fn new(interpreter: Interpreter) -> Self {
        Self {
            interpreter,
            visited_rooms: HashSet::new(),
            items: Vec::new(),
        }
    }

    fn take(&mut self, idx: usize) {
        self.interpreter.input.extend(from_ascii("take "));
        self.interpreter.input.extend(from_ascii(&self.items[idx]));
        self.interpreter.input.push_back(NL);
        self.interpreter.run();
        self.interpreter.output.clear();
    }

    fn drop(&mut self, idx: usize) {
        self.interpreter.input.extend(from_ascii("drop "));
        self.interpreter.input.extend(from_ascii(&self.items[idx]));
        self.interpreter.input.push_back(NL);
        self.interpreter.run();
        self.interpreter.output.clear();
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
        for item in items {
            if is_safe(&item) {
                self.items.push(item);
                self.take(self.items.len() - 1);
            }
        }

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
    program.extend_from_slice(&[Cell::Value(0); 225]);
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

    // Drop all known items
    (0..droid.items.len()).for_each(|item| droid.drop(item));

    // For all items, iterated in reverse so that we can remove
    for item in (0..droid.items.len()).rev() {
        // Get the item from the floor
        droid.take(item);

        // Enter the pressure-sensitive room
        droid.enter(Door::West);
        let weight = analyse(&droid.interpreter.output);
        droid.interpreter.output.clear();

        // Drop the item
        droid.drop(item);

        // If the item makes us too heavy on its own, remove it from consideration
        if weight == Weight::TooHeavy {
            droid.items.remove(item);
        }
    }

    // Pick everything back up
    (0..droid.items.len()).for_each(|item| droid.take(item));

    for item in (0..droid.items.len()).rev() {
        // Drop just one item at a time
        droid.drop(item);

        // Analyse our weight with everything but that item
        droid.enter(Door::West);
        let weight = analyse(&droid.interpreter.output);
        droid.interpreter.output.clear();

        // Pick it back up
        droid.take(item);

        // If we're too light without the item, we need the item to reach the target weight, so
        // just always keep it and never drop it again
        if weight == Weight::TooLight {
            droid.items.remove(item);
        }
    }

    // Now, armed with our knowledge of what we mustn't drop ...
    // Check every possible item set, dropping some of the optional items every time
    for i in 0u8..(1 << droid.items.len()) {
        for item in 0..droid.items.len() {
            if i & (1 << item) != 0 {
                droid.drop(item);
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
        for item in 0..droid.items.len() {
            if i & (1 << item) != 0 {
                droid.take(item);
            }
        }
    }
}

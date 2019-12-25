use std::collections::VecDeque;
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl TryFrom<i64> for ParameterMode {
    type Error = ();

    fn try_from(n: i64) -> Result<Self, Self::Error> {
        match n {
            0 => Ok(Self::Position),
            1 => Ok(Self::Immediate),
            2 => Ok(Self::Relative),
            _ => Err(()),
        }
    }
}

impl From<ParameterMode> for i64 {
    fn from(mode: ParameterMode) -> i64 {
        match mode {
            ParameterMode::Position => 0,
            ParameterMode::Immediate => 1,
            ParameterMode::Relative => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Instruction(ParameterMode, ParameterMode, ParameterMode, u8),
    Value(i64),
}

impl Cell {
    fn to_instruction(&mut self) {
        let mut value = match self {
            Cell::Value(value) => *value,
            Cell::Instruction(..) => return,
        };

        let opcode = value % 100;

        match opcode {
            1..=9 | 99 => {
                let mut doit = || {
                    value /= 100;

                    let a = ParameterMode::try_from(value % 10).ok()?;
                    value /= 10;

                    let b = ParameterMode::try_from(value % 10).ok()?;
                    value /= 10;

                    let c = ParameterMode::try_from(value % 10).ok()?;
                    value /= 10;

                    Some((a, b, c))
                };

                if let Some((a, b, c)) = doit() {
                    if value == 0 {
                        *self = Cell::Instruction(a, b, c, opcode as _);
                    }
                }
            }

            _ => {}
        }
    }
}

impl From<Cell> for i64 {
    fn from(cell: Cell) -> i64 {
        match cell {
            Cell::Instruction(a, b, c, opcode) => {
                (i64::from(c) * 100 + i64::from(b) * 10 + i64::from(a)) * 100 + opcode as i64
            }
            Cell::Value(value) => value,
        }
    }
}

#[derive(Debug)]
pub struct Interpreter {
    pub program: Vec<Cell>,
    pub input: VecDeque<i64>,
    pub output: VecDeque<i64>,

    pub pc: usize,

    pub done: bool,

    pub relative_base: i64,
}

impl Interpreter {
    pub fn new(program: Vec<Cell>) -> Self {
        Self {
            program,
            input: VecDeque::new(),
            output: VecDeque::new(),
            pc: 0,
            done: false,
            relative_base: 0,
        }
    }

    pub fn run(&mut self) {
        macro_rules! position {
            ($mode:expr, $cell:expr) => {{
                match $mode {
                    ParameterMode::Position => i64::from($cell),
                    ParameterMode::Relative => i64::from($cell) + self.relative_base,
                    ParameterMode::Immediate => unreachable!(),
                }
            }};
        }

        macro_rules! load {
            ($mode:expr, $cell:expr) => {{
                let cell = i64::from($cell);

                match $mode {
                    mode @ ParameterMode::Position | mode @ ParameterMode::Relative => {
                        let idx = position!(mode, cell);

                        i64::from(self.program[idx as usize])
                    }

                    ParameterMode::Immediate => cell,
                }
            }};
        };

        macro_rules! store {
            ($idx:expr, $value:expr) => {
                let idx = $idx;

                self.program[idx as usize] = Cell::Value($value);
            };
        }

        macro_rules! next_cell {
            () => {{
                let cell = self.program[self.pc];
                self.pc += 1;
                cell
            }};
        }

        loop {
            let old_pc = self.pc;

            self.program[self.pc].to_instruction();

            match next_cell!() {
                Cell::Instruction(a, b, c, 1) => {
                    let a = load!(a, next_cell!());
                    let b = load!(b, next_cell!());
                    let c = position!(c, next_cell!());

                    store!(c, a + b);
                }

                Cell::Instruction(a, b, c, 2) => {
                    let a = load!(a, next_cell!());
                    let b = load!(b, next_cell!());
                    let c = position!(c, next_cell!());

                    store!(c, a * b);
                }

                Cell::Instruction(
                    ParameterMode::Position,
                    ParameterMode::Position,
                    ParameterMode::Position,
                    99,
                ) => {
                    self.pc = old_pc;
                    self.done = true;
                    break;
                }

                Cell::Instruction(a, ParameterMode::Position, ParameterMode::Position, 3) => {
                    let a = position!(a, next_cell!());

                    if let Some(input) = self.input.pop_front() {
                        store!(a, input);
                    } else {
                        self.pc = old_pc;
                        break;
                    }
                }

                Cell::Instruction(a, ParameterMode::Position, ParameterMode::Position, 4) => {
                    let a = load!(a, next_cell!());

                    self.output.push_back(a);
                }

                Cell::Instruction(a, b, ParameterMode::Position, 5) => {
                    let a = load!(a, next_cell!());
                    let b = load!(b, next_cell!());

                    if a != 0 {
                        self.pc = b as usize;
                    }
                }

                Cell::Instruction(a, b, ParameterMode::Position, 6) => {
                    let a = load!(a, next_cell!());
                    let b = load!(b, next_cell!());

                    if a == 0 {
                        self.pc = b as usize;
                    }
                }

                Cell::Instruction(a, b, c, 7) => {
                    let a = load!(a, next_cell!());
                    let b = load!(b, next_cell!());
                    let c = position!(c, next_cell!());

                    store!(c, if a < b { 1 } else { 0 });
                }

                Cell::Instruction(a, b, c, 8) => {
                    let a = load!(a, next_cell!());
                    let b = load!(b, next_cell!());
                    let c = position!(c, next_cell!());

                    store!(c, if a == b { 1 } else { 0 });
                }

                Cell::Instruction(a, ParameterMode::Position, ParameterMode::Position, 9) => {
                    let a = load!(a, next_cell!());

                    self.relative_base += a;
                }

                _ => {
                    self.pc = old_pc;
                    panic!("unknown opcode {:?}", self.program[self.pc])
                }
            }
        }
    }
}

pub fn load_program(input: &str) -> Vec<Cell> {
    input
        .trim()
        .split(',')
        .map(|n| Cell::Value(n.parse().unwrap()))
        .collect()
}

pub fn from_ascii<'a>(input: &'a str) -> impl Iterator<Item = i64> + 'a {
    input.bytes().map(|c| c as i64)
}

pub fn to_ascii(input: impl IntoIterator<Item = i64>) -> impl Iterator<Item = char> {
    input.into_iter().map(|c| c as u8 as char)
}

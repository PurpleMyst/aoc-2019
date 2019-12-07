use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParameterMode {
    Position,
    Immediate,
}

impl TryFrom<isize> for ParameterMode {
    type Error = ();

    fn try_from(n: isize) -> Result<Self, Self::Error> {
        match n {
            0 => Ok(Self::Position),
            1 => Ok(Self::Immediate),
            _ => Err(()),
        }
    }
}

impl From<ParameterMode> for isize {
    fn from(mode: ParameterMode) -> isize {
        match mode {
            ParameterMode::Position => 0,
            ParameterMode::Immediate => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Instruction(ParameterMode, ParameterMode, ParameterMode, u8),
    Value(isize),
}

impl From<isize> for Cell {
    fn from(mut value: isize) -> Cell {
        let orig_value = value;

        let opcode = value % 100;

        match opcode {
            1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 99 => {
                let mut doit = || {
                    value /= 100;

                    let a_mode = ParameterMode::try_from(value % 10)?;
                    value /= 10;

                    let b_mode = ParameterMode::try_from(value % 10)?;
                    value /= 10;

                    let c_mode = ParameterMode::try_from(value % 10)?;
                    value /= 10;

                    if value == 0 {
                        Ok(Cell::Instruction(a_mode, b_mode, c_mode, opcode as _))
                    } else {
                        Err(())
                    }
                };

                doit().unwrap_or(Cell::Value(orig_value))
            }

            _ => Cell::Value(orig_value),
        }
    }
}

impl From<Cell> for isize {
    fn from(cell: Cell) -> isize {
        match cell {
            Cell::Instruction(a, b, c, opcode) => {
                (isize::from(c) * 1000 + isize::from(b) * 100 + isize::from(a) * 10) * 10
                    + opcode as isize
            }
            Cell::Value(value) => value,
        }
    }
}

pub fn interpret(program: &mut [Cell], input: isize) -> Vec<isize> {
    let mut pc = 0;

    let mut output = Vec::new();

    macro_rules! load {
        ($mode:expr, $cell:expr) => {{
            let cell = isize::from($cell);

            match $mode {
                ParameterMode::Position => isize::from(program[cell as usize]),
                ParameterMode::Immediate => cell,
            }
        }};
    };

    macro_rules! store {
        ($position:expr, $value:expr) => {
            program[isize::from($position) as usize] = Cell::from($value);
        };
    }

    macro_rules! next_cell {
        () => {{
            let arg = program[pc];
            pc += 1;
            arg
        }};
    }

    loop {
        match next_cell!() {
            Cell::Instruction(a, b, ParameterMode::Position, 1) => {
                let a = load!(a, next_cell!());
                let b = load!(b, next_cell!());
                let c = next_cell!();

                store!(c, a + b);
            }

            Cell::Instruction(a, b, ParameterMode::Position, 2) => {
                let a = load!(a, next_cell!());
                let b = load!(b, next_cell!());
                let c = next_cell!();

                store!(c, a * b);
            }

            Cell::Instruction(
                ParameterMode::Position,
                ParameterMode::Position,
                ParameterMode::Position,
                99,
            ) => return output,

            Cell::Instruction(
                ParameterMode::Position,
                ParameterMode::Position,
                ParameterMode::Position,
                3,
            ) => {
                let a = next_cell!();

                store!(a, Cell::from(input));
            }

            Cell::Instruction(a, ParameterMode::Position, ParameterMode::Position, 4) => {
                let a = load!(a, next_cell!());

                output.push(a);
            }

            Cell::Instruction(a, b, ParameterMode::Position, 5) => {
                let a = load!(a, next_cell!());
                let b = load!(b, next_cell!());

                if a != 0 {
                    pc = b as usize;
                }
            }

            Cell::Instruction(a, b, ParameterMode::Position, 6) => {
                let a = load!(a, next_cell!());
                let b = load!(b, next_cell!());

                if a == 0 {
                    pc = b as usize;
                }
            }

            Cell::Instruction(a, b, ParameterMode::Position, 7) => {
                let a = load!(a, next_cell!());
                let b = load!(b, next_cell!());
                let c = next_cell!();

                store!(c, if a < b { 1 } else { 0 });
            }

            Cell::Instruction(a, b, ParameterMode::Position, 8) => {
                let a = load!(a, next_cell!());
                let b = load!(b, next_cell!());
                let c = next_cell!();

                store!(c, if a == b { 1 } else { 0 });
            }

            _ => unreachable!("tried to execute {:?}", isize::from(program[pc - 1])),
        }
    }
}

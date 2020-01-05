use std::{collections::VecDeque, hint::unreachable_unchecked};

#[derive(Debug, Clone)]
pub struct Interpreter {
    pub memory: Vec<i64>,

    pub input: VecDeque<i64>,
    pub output: VecDeque<i64>,

    pub pc: usize,

    pub done: bool,

    pub relative_base: i64,
}

type Opcode = fn(&mut Interpreter) -> ();

pub const JUMP_TABLE: [Option<Opcode>; 22209] = {
    let mut jump_table = [None; 22209];

    /// Call the given triadic macro with each triplet in the cartesian product of the arguments
    macro_rules! cartesian_product {
        ($submac:ident <- ($x:ident $($xs:ident)*; $y:ident $($ys:ident)*; $z:ident $($zs:ident)*)) => {
            $submac!($x, $y, $z);
            cartesian_product!($submac <- ($x; $y; $($zs)*));
            cartesian_product!($submac <- ($x; $($ys)*; $z $($zs)*));
            cartesian_product!($submac <- ($($xs)*; $y $($ys)*; $z $($zs)*));
        };

        ($submac:ident <- ($($xs:ident)*; $($ys:ident)*; $($zs:ident)*)) => {};
    }

    // NB: All the loading routines are macros due to borrowck issues. Namely, if we make the `*_mut`
    // routines methods on `Interpreter` the input opcode has issues with popping from the input

    // All the unsafes are to avoid bounds checking in release mode

    /// Load the current parameter as an absolute address
    macro_rules! absolute {
        ($interpreter:ident) => {
            unsafe {
                let idx = *$interpreter.memory.get_unchecked($interpreter.pc);
                $interpreter.pc += 1;
                *$interpreter.memory.get_unchecked(idx as usize)
            }
        };
    }

    /// Load the current parameter as an immediate value
    macro_rules! immediate {
        ($interpreter:ident) => {
            unsafe {
                let value = *$interpreter.memory.get_unchecked($interpreter.pc);
                $interpreter.pc += 1;
                value
            }
        };
    }

    /// Load the current parameter as a relative address
    macro_rules! relative {
        ($interpreter:ident) => {
            unsafe {
                let idx = $interpreter.relative_base
                    + *$interpreter.memory.get_unchecked($interpreter.pc);
                $interpreter.pc += 1;
                *$interpreter.memory.get_unchecked(idx as usize)
            }
        };
    }

    /// Load the current parameter as a mutable absolute address
    macro_rules! absolute_mut {
        ($interpreter:ident) => {
            unsafe {
                let idx = *$interpreter.memory.get_unchecked($interpreter.pc);
                $interpreter.pc += 1;
                $interpreter.memory.get_unchecked_mut(idx as usize)
            }
        };
    }

    /// Load the current parameter as a mutable relative address
    macro_rules! relative_mut {
        ($interpreter:ident) => {
            unsafe {
                let idx = $interpreter.relative_base
                    + *$interpreter.memory.get_unchecked($interpreter.pc);
                $interpreter.pc += 1;
                $interpreter.memory.get_unchecked_mut(idx as usize)
            }
        };
    }

    /// Calculate the leading three digits of an opcode with the given loading modes
    macro_rules! mode {
        ($a:ident, $b:ident, $c:ident) => {
            (mode!(@doit $c) * 100 + mode!(@doit $b) * 10 + mode!(@doit $a)) * 100
        };

        (@doit absolute)     => { 0 };
        (@doit absolute_mut) => { 0 };
        (@doit immediate)    => { 1 };
        (@doit relative)     => { 2 };
        (@doit relative_mut) => { 2 };
    }

    /// Add an opcode to the jump table with all of its corresponding mode combinations
    macro_rules! add_opcode {
        // triadic instruction which loads from first two arguments and stores into third
        ($opcode:literal => |$interpreter:ident, $a_var:ident, $b_var:ident, $c_var:ident| $body:expr) => {
            macro_rules! helper {
                ($a:ident, $b:ident, $c:ident) => {
                    jump_table[mode!($a, $b, $c) + $opcode] = Some((|$interpreter| {
                        debug_assert!($interpreter.pc + 3 < $interpreter.memory.len());
                        let $a_var = $a!($interpreter);
                        let $b_var = $b!($interpreter);
                        let $c_var = $c!($interpreter);
                        $body;
                        $interpreter.run();
                    }) as Opcode);
                };
            }

            cartesian_product!(helper <- (absolute immediate relative; absolute immediate relative; absolute_mut relative_mut));
        };

        // dyadic instruction which loads from first two arguments
        ($opcode:literal => |$interpreter:ident, $a_var:ident, $b_var:ident| $body:expr) => {
            macro_rules! helper {
                ($a:ident, $b:ident, absolute) => {
                    jump_table[mode!($a, $b, absolute) + $opcode] = Some((|$interpreter| {
                        debug_assert!($interpreter.pc + 2 < $interpreter.memory.len());
                        let $a_var = $a!($interpreter);
                        let $b_var = $b!($interpreter);
                        $body;
                        $interpreter.run();
                    }) as Opcode);
                };
            }

            cartesian_product!(helper <- (absolute immediate relative; absolute immediate relative; absolute));
        };

        // monadic instruction which loads from first argument
        ($opcode:literal => |$interpreter:ident, $a_var:ident| $body:expr) => {
            macro_rules! helper {
                ($a:ident, absolute, absolute) => {
                    jump_table[mode!($a, absolute, absolute) + $opcode] = Some((|$interpreter| {
                        debug_assert!($interpreter.pc + 1 < $interpreter.memory.len());
                        let $a_var = $a!($interpreter);
                        $body;
                        $interpreter.run();
                    }) as Opcode);
                };
            }

            cartesian_product!(helper <- (absolute immediate relative; absolute; absolute));
        };

        // monadic instruction which stores into argument
        ($opcode:literal => |$interpreter:ident, &mut $a_var:ident| $body:expr) => {
            macro_rules! helper {
                ($a:ident, absolute, absolute) => {
                    jump_table[mode!($a, absolute, absolute) + $opcode] = Some((|$interpreter| {
                        debug_assert!($interpreter.pc + 1 < $interpreter.memory.len());
                        let $a_var = $a!($interpreter);
                        $body;
                        $interpreter.run();
                    }) as Opcode);
                };
            }

            cartesian_product!(helper <- (absolute_mut relative_mut; absolute; absolute));
        };

        // NB: No `interpreter.run()` is present after the $body because we use this just for 99
        // nulladic instruction
        ($opcode:literal => |$interpreter:ident| $body:expr) => {
            jump_table[$opcode] = Some((|$interpreter| { $body }) as Opcode);
        };
    }

    add_opcode!(1 => |interp, a, b, c| { *c = a + b });
    add_opcode!(2 => |interp, a, b, c| { *c = a * b });
    add_opcode!(3 => |interp, &mut a| if let Some(input) = interp.input.pop_front() { *a = input; } else { interp.pc -= 2; return; });
    add_opcode!(4 => |interp, a| interp.output.push_back(a));
    add_opcode!(5 => |interp, a, b| if a != 0 { interp.pc = b as usize; });
    add_opcode!(6 => |interp, a, b| if a == 0 { interp.pc = b as usize; });
    add_opcode!(7 => |interp, a, b, c| *c = if a < b { 1 } else { 0 });
    add_opcode!(8 => |interp, a, b, c| *c = if a == b { 1 } else { 0 });
    add_opcode!(9 => |interp, a| interp.relative_base += a);

    add_opcode!(99 => |interp| { interp.pc -= 1; interp.done = true; return; });

    jump_table
};

impl Interpreter {
    pub fn new(memory: Vec<i64>) -> Self {
        Self {
            memory,
            input: VecDeque::new(),
            output: VecDeque::new(),
            pc: 0,
            done: false,
            relative_base: 0,
        }
    }

    pub fn run(&mut self) {
        debug_assert!(JUMP_TABLE[self.memory[self.pc] as usize].is_some());

        unsafe {
            let opcode = *self.memory.get_unchecked(self.pc);
            self.pc += 1;
            match JUMP_TABLE.get_unchecked(opcode as usize) {
                Some(opcode) => opcode(self),
                None => unreachable_unchecked(),
            }
        }
    }

    pub fn from_input(input: &str) -> Self {
        Self::new(
            input
                .trim()
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect(),
        )
    }

    pub fn input_from_ascii(&mut self, input: &str) {
        self.input.extend(input.bytes().map(|c| c as i64));
    }

    pub fn output_as_ascii(&self) -> impl Iterator<Item = char> + '_ {
        self.input.iter().copied().map(|c| c as u8 as char)
    }
}

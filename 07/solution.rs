include!("../intcode.rs");

type Phases = (isize, isize, isize, isize, isize);

fn amplify(program: &Vec<Cell>, (a, b, c, d, e): Phases) -> isize {
    let mut interpreters = [
        Interpreter::new(program.clone()),
        Interpreter::new(program.clone()),
        Interpreter::new(program.clone()),
        Interpreter::new(program.clone()),
        Interpreter::new(program.clone()),
    ];

    interpreters[0].input.push_back(a);
    interpreters[1].input.push_back(b);
    interpreters[2].input.push_back(c);
    interpreters[3].input.push_back(d);
    interpreters[4].input.push_back(e);

    interpreters[0].input.push_back(0);

    while !interpreters.last().unwrap().done {
        for i in 0..interpreters.len() {
            interpreters[i].run();

            if let Some(output) = interpreters[i].output.pop_front() {
                interpreters[(i + 1) % interpreters.len()]
                    .input
                    .push_back(output);
            }
        }
    }

    interpreters[0].input[0]
}

fn for_all_phases(left: isize, right: isize, mut f: impl FnMut(Phases) -> ()) {
    for a in left..=right {
        for b in left..=right {
            if !(b != a) {
                continue;
            }
            for c in left..=right {
                if !(c != a && c != b) {
                    continue;
                }
                for d in left..=right {
                    if !(d != a && d != b && d != c) {
                        continue;
                    }
                    for e in left..=right {
                        if !(e != a && e != b && e != c && e != d) {
                            continue;
                        }

                        f((a, b, c, d, e))
                    }
                }
            }
        }
    }
}

fn largest_output(program: &Vec<Cell>, low: isize, high: isize) -> isize {
    let mut result = 0;
    for_all_phases(low, high, |phases| {
        let output = isize::from(amplify(&program, phases));

        if output > result {
            result = output;
        }
    });
    result
}

fn main() {
    let program = load_program(include_str!("input.txt"));

    println!("{}", largest_output(&program, 0, 4));

    println!("{}", largest_output(&program, 5, 9));
}

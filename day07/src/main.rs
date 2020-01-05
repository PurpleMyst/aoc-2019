use intcode::Interpreter;

type Phases = (i64, i64, i64, i64, i64);

fn amplify(interpreter: &Interpreter, (a, b, c, d, e): Phases) -> i64 {
    let mut interpreters = [
        interpreter.clone(),
        interpreter.clone(),
        interpreter.clone(),
        interpreter.clone(),
        interpreter.clone(),
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

fn for_all_phases(left: i64, right: i64, mut f: impl FnMut(Phases) -> ()) {
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

fn largest_output(interpreter: &Interpreter, low: i64, high: i64) -> i64 {
    let mut result = 0;
    for_all_phases(low, high, |phases| {
        let output = i64::from(amplify(&interpreter, phases));

        if output > result {
            result = output;
        }
    });
    result
}

fn main() {
    let interpreter = Interpreter::from_input(include_str!("input.txt"));
    println!("{}", largest_output(&interpreter, 0, 4));
    println!("{}", largest_output(&interpreter, 5, 9));
}

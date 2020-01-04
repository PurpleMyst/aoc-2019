fn load_input() -> (usize, usize) {
    let mut it = include_str!("input.txt").trim().split('-');
    let low = it.next().unwrap().parse::<usize>().unwrap();
    let high = it.next().unwrap().parse::<usize>().unwrap();
    (low, high)
}

fn main() {
    let (low, high) = load_input();

    let mut part1 = 0;
    let mut part2 = 0;

    for a in 0..=9 {
        for b in a..=9 {
            for c in b..=9 {
                for d in c..=9 {
                    for e in d..=9 {
                        for f in e..=9 {
                            let n = a * 100000 + b * 10000 + c * 1000 + d * 100 + e * 10 + f * 1;
                            if n < low || n > high {
                                continue;
                            }

                            let mut counts = [0; 10];
                            counts[a] += 1;
                            counts[b] += 1;
                            counts[c] += 1;
                            counts[d] += 1;
                            counts[e] += 1;
                            counts[f] += 1;

                            if counts.iter().copied().any(|n| n >= 2) {
                                part1 += 1;
                            }

                            if counts.iter().copied().any(|n| n == 2) {
                                part2 += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}", part1);
    println!("{}", part2);
}

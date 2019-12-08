use std::cmp::min;

// TODO: minimize the sizes of the integers used here; that could potentially help with cache

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct HorizontalSegment {
    x: isize,
    y: isize,
    length: isize,
    idx: isize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct VerticalSegment {
    x: isize,
    y: isize,
    length: isize,
    idx: isize,
}

fn points(path: &str) -> (Vec<HorizontalSegment>, Vec<VerticalSegment>) {
    let mut horizontal = Vec::new();
    let mut vertical = Vec::new();

    let mut x = 0;
    let mut y = 0;
    let mut idx = 0;

    for step in path.split(',') {
        let mut step = step.chars();

        let direction: char = step.next().unwrap();
        let length: isize = step.as_str().parse().unwrap();

        match direction {
            'U' => {
                vertical.push(VerticalSegment {
                    x,
                    y,
                    length: -length,
                    idx,
                });
                y -= length;
            }

            'D' => {
                vertical.push(VerticalSegment { x, y, length, idx });
                y += length;
            }

            'L' => {
                horizontal.push(HorizontalSegment {
                    x,
                    y,
                    length: -length,
                    idx,
                });
                x -= length;
            }

            'R' => {
                horizontal.push(HorizontalSegment { x, y, length, idx });
                x += length;
            }

            _ => unreachable!(),
        }

        idx += length;
    }

    (horizontal, vertical)
}

fn intersect_1d(l: isize, h: isize, x: isize) -> bool {
    (x >= l && x <= h) || (x >= h && x <= l)
}

fn intersect(hor: HorizontalSegment, vert: VerticalSegment) -> Option<(isize, isize)> {
    if intersect_1d(hor.x, hor.x + hor.length, vert.x)
        && intersect_1d(vert.y, vert.y + vert.length, hor.y)
    {
        Some((vert.x, hor.y))
    } else {
        None
    }
}

fn main() {
    let mut wires = include_str!("input.txt").trim().lines().map(points);

    let (wire1_hor, wire1_vert) = wires.next().unwrap();
    let (wire2_hor, wire2_vert) = wires.next().unwrap();

    let mut part1 = 10_000;
    let mut part2 = 10_000_000;

    macro_rules! doit {
        ($hor:expr; $vert:expr) => {
            for hor in $hor.iter().copied() {
                for vert in $vert.iter().copied() {
                    if let Some((x, y)) = intersect(hor, vert) {
                        if (x, y) != (0, 0) {
                            part1 = min(part1, x.abs() + y.abs());

                            let steps1 = hor.idx + (y - vert.y).abs();
                            let steps2 = vert.idx + (x - hor.x).abs();
                            part2 = min(part2, steps1 + steps2);

                            break;
                        }
                    }
                }
            }
        };
    }

    doit!(wire1_hor; wire2_vert);
    doit!(wire2_hor; wire1_vert);

    println!("{}", part1);
    println!("{}", part2);
}

use std::cmp::min;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct HorizontalSegment {
    x: i32,
    y: i32,
    length: i32,
    idx: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct VerticalSegment {
    x: i32,
    y: i32,
    length: i32,
    idx: i32,
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
        let length: i32 = step.as_str().parse().unwrap();

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

fn intersect_1d(l: i32, h: i32, x: i32) -> bool {
    (x >= l && x <= h) || (x >= h && x <= l)
}

fn intersect(horizontal: HorizontalSegment, vertical: VerticalSegment) -> Option<(i32, i32)> {
    if intersect_1d(horizontal.x, horizontal.x + horizontal.length, vertical.x)
        && intersect_1d(vertical.y, vertical.y + vertical.length, horizontal.y)
    {
        Some((vertical.x, horizontal.y))
    } else {
        None
    }
}

fn solve(horizontal: Vec<HorizontalSegment>, vertical: Vec<VerticalSegment>) -> (i32, i32) {
    let mut part1 = std::i32::MAX;
    let mut part2 = std::i32::MAX;

    for horizontal in horizontal {
        for &vertical in &vertical {
            if let Some((x, y)) = intersect(horizontal, vertical) {
                if (x, y) != (0, 0) {
                    part1 = min(part1, x.abs() + y.abs());

                    let steps1 = horizontal.idx + (y - vertical.y).abs();
                    let steps2 = vertical.idx + (x - horizontal.x).abs();
                    part2 = min(part2, steps1 + steps2);

                    break;
                }
            }
        }
    }

    (part1, part2)
}

fn main() {
    let mut wires = include_str!("input.txt").trim().lines().map(points);

    let (wire1_horizontal, wire1_vertical) = wires.next().unwrap();
    let (wire2_horizontal, wire2_vertical) = wires.next().unwrap();

    let (part1_1, part2_1) = solve(wire1_horizontal, wire2_vertical);
    let (part1_2, part2_2) = solve(wire2_horizontal, wire1_vertical);

    println!("{}", min(part1_1, part1_2));
    println!("{}", min(part2_1, part2_2));
}

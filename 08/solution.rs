use std::collections::BTreeMap;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

const BLACK: u8 = b'0';
const WHITE: u8 = b'1';
const TRANSPARENT: u8 = b'2';

fn count(arr: &[u8]) -> BTreeMap<u8, usize> {
    let mut result = BTreeMap::new();

    arr.iter()
        .copied()
        .for_each(|n| *result.entry(n).or_default() += 1);

    result
}

fn main() {
    let layers = include_str!("input.txt")
        .trim()
        .as_bytes()
        .chunks(WIDTH * HEIGHT);

    let layer = layers
        .clone()
        .map(count)
        .min_by_key(|layer| *layer.get(&b'0').unwrap())
        .unwrap();

    println!("{}", layer.get(&b'1').unwrap() * layer.get(&b'2').unwrap());

    let mut image = [TRANSPARENT; WIDTH * HEIGHT];

    layers.for_each(|layer| {
        layer.iter().copied().enumerate().for_each(|(i, pixel)| {
            if image[i] == TRANSPARENT {
                image[i] = pixel;
            }
        })
    });

    image.chunks(WIDTH).for_each(|row| {
        row.iter().copied().for_each(|col| {
            print!(
                "{}",
                match col {
                    BLACK => ' ',
                    WHITE => '#',
                    _ => unreachable!(),
                }
            );
        });

        println!();
    })
}

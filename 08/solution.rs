use std::collections::BTreeMap;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

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
        .map(count)
        .min_by_key(|layer| *layer.get(&b'0').unwrap())
        .unwrap();

    println!("{}", layer.get(&b'1').unwrap() * layer.get(&b'2').unwrap())
}

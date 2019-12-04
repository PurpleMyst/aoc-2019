fn load_input() -> (usize, usize) {
    let mut it = include_str!("input.txt").trim().split('-');
    let low = it.next().unwrap().parse::<usize>().unwrap();
    let high = it.next().unwrap().parse::<usize>().unwrap();
    assert!(it.next().is_none());
    (low, high)
}

fn is_sorted(s: &[u8]) -> bool {
    (0..s.len() - 1).all(|i| s[i] <= s[i + 1])
}

mod first {
    pub(super) fn has_pair(s: &[u8]) -> bool {
        s.windows(2).any(|win| win[0] == win[1])
    }
}

mod second {
    pub(super) fn has_pair(s: &[u8]) -> bool {
        let mut cur = 0;
        let mut count = 0;

        for c in s.iter().copied() {
            if cur != c {
                if count == 2 {
                    return true;
                }

                cur = c;
                count = 1;
            } else {
                count += 1;
            }
        }

        count == 2
    }
}

fn main() {
    let (low, high) = load_input();

    println!(
        "{}",
        (low..high)
            .map(|n| n.to_string())
            .filter(|s| is_sorted(s.as_bytes()))
            .filter(|s| first::has_pair(s.as_bytes()))
            .count()
    );

    println!(
        "{}",
        (low..high)
            .map(|n| n.to_string())
            .filter(|s| is_sorted(s.as_bytes()))
            .filter(|s| second::has_pair(s.as_bytes()))
            .count()
    );
}

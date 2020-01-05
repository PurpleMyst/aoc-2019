const P1_CARDS: i64 = 10007;

const P2_CARDS: i64 = 119315717514047;
const P2_REPETITIONS: i64 = 101741582076661;

fn mulmod(mut a: i64, mut b: i64, m: i64) -> i64 {
    a = a.rem_euclid(m);

    if b < 0 {
        b = m + b;
    }

    let mut res = 0;
    while b != 0 {
        if b & 1 != 0 {
            res = (res + a).rem_euclid(m);
        }
        a = (2 * a).rem_euclid(m);
        b >>= 1;
    }
    return res;
}

fn powmod(mut x: i64, mut y: i64, p: i64) -> i64 {
    let mut res = 1;

    x = x.rem_euclid(p);

    while y > 0 {
        if y & 1 != 0 {
            res = mulmod(res, x, p);
        }

        y >>= 1;
        x = mulmod(x, x, p);
    }

    return res;
}

fn invmod(a: i64, m: i64) -> i64 {
    powmod(a, m - 2, m)
}

struct LCF(i64, i64);

impl LCF {
    fn identity() -> Self {
        Self(1, 0)
    }

    fn compose(self, other: Self, m: i64) -> Self {
        Self(
            mulmod(self.0, other.0, m),
            (mulmod(self.1, other.0, m) + other.1).rem_euclid(m),
        )
    }

    fn apply(self, value: i64, m: i64) -> i64 {
        (mulmod(self.0, value, m) + self.1).rem_euclid(m)
    }

    fn repeat(self, k: i64, m: i64) -> Self {
        let a_k = powmod(self.0, k, m);
        Self(
            a_k,
            mulmod(self.1, mulmod(1 - a_k, invmod(1 - self.0, m), m), m),
        )
    }

    fn inverse(self, m: i64) -> Self {
        let a_inv = invmod(self.0, m);
        Self(a_inv, -(mulmod(self.1, a_inv, m)))
    }
}

fn get_input(m: i64) -> LCF {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            if line == "deal into new stack" {
                LCF(-1, -1)
            } else {
                let n = line.split(" ").last().unwrap().parse::<i64>().unwrap();

                if line.starts_with("cut") {
                    LCF(1, -n)
                } else {
                    LCF(n, 0)
                }
            }
        })
        .fold(LCF::identity(), |a, b| a.compose(b, m))
}

fn main() {
    println!("{}", get_input(P1_CARDS).apply(2019, P1_CARDS));

    println!(
        "{}",
        get_input(P2_CARDS)
            .repeat(P2_REPETITIONS, P2_CARDS)
            .inverse(P2_CARDS)
            .apply(2020, P2_CARDS)
    );
}

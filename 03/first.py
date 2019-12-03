#!/usr/bin/env python3
import typing as t


def points(path: str) -> t.Set[complex]:
    pos = 0j
    result = set()

    for step in path.split(","):
        direction = {"R": 1, "L": -1, "U": -1j, "D": 1j}[step[0]]
        amount = int(step[1:])

        for _ in range(amount):
            pos += direction
            result.add(pos)

    return result


def main() -> None:
    with open("input.txt") as f:
        w1, w2 = map(points, f)
        d = min(int(abs(p.real) + abs(p.imag)) for p in w1 & w2)
        print(d)


if __name__ == "__main__":
    main()

#!/usr/bin/env python3
import typing as t


def points(path: str) -> t.Dict[complex, int]:
    pos = 0j
    i = 1
    distances = {}

    for step in path.split(","):
        direction = {"R": 1, "L": -1, "U": -1j, "D": 1j}[step[0]]
        amount = int(step[1:])

        for _ in range(amount):
            pos += direction
            distances.setdefault(pos, i)
            i += 1

    return distances


def main() -> None:
    with open("input.txt") as f:
        w1, w2 = map(points, f)
        print(min(w1[k] + w2[k] for k in w1.keys() & w2.keys()))


if __name__ == "__main__":
    main()

#!/usr/bin/env python3
import itertools

from first import solve


def has_pair(s: str) -> bool:
    return any(len(tuple(group)) == 2 for _, group in itertools.groupby(s))


def main() -> None:
    print(solve(has_pair))


if __name__ == "__main__":
    main()

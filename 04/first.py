#!/usr/bin/env python3
import itertools
import typing as t


def is_sorted(s: str) -> bool:
    for i, c in enumerate(s[:-1]):
        if c > s[i + 1]:
            return False
    return True


def has_pair(s: str) -> bool:
    return any(len(tuple(group)) >= 2 for _, group in itertools.groupby(s))


def load_input() -> t.Iterable[str]:
    with open("input.txt") as f:
        return map(str, range(*map(int, f.read().split("-"))))


def solve(has_pair: t.Callable[[str], bool]) -> int:
    return len(tuple(filter(has_pair, filter(is_sorted, load_input()))))


def main() -> None:
    print(solve(has_pair))


if __name__ == "__main__":
    main()

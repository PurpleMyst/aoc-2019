#!/usr/bin/env python3
import itertools
import typing as t


def is_sorted(s: t.Iterable[t.Any]) -> bool:
    it = iter(s)

    i = next(it)
    for j in it:
        if i > j:
            return False

        i = j

    return True


def has_pair(s: str) -> bool:
    for _, group in itertools.groupby(s):
        try:
            next(group)
            next(group)
        except StopIteration:
            pass
        else:
            return True

    return False


def load_input() -> t.Iterable[str]:
    with open("input.txt") as f:
        return map(str, range(*map(int, f.read().split("-"))))


def solve(has_pair: t.Callable[[str], bool]) -> int:
    return len(tuple(filter(has_pair, filter(is_sorted, load_input()))))


def main() -> None:
    print(solve(has_pair))


if __name__ == "__main__":
    main()

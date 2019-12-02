#!/usr/bin/env python3


def fuel(mass: int) -> int:
    return mass // 3 - 2


def main() -> None:
    with open("input.txt") as inp:
        print(sum(fuel(int(mass)) for mass in inp))


if __name__ == "__main__":
    main()

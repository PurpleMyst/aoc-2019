#!/usr/bin/env python3
from first import fuel


def total_fuel(mass: int) -> int:
    f = fuel(mass)
    if f > 0:
        return f + total_fuel(f)
    else:
        return 0


def main() -> None:
    with open("input.txt") as inp:
        print(sum(total_fuel(int(mass)) for mass in inp))


if __name__ == "__main__":
    main()

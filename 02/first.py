#!/usr/bin/env python3
import typing as t

ADD = 1
MUL = 2
HALT = 99


def intcode(program: t.List[int]) -> t.List[int]:
    pc = 0
    while program[pc] != HALT:
        a, b, c = program[pc + 1 : pc + 4]

        if program[pc] == ADD:
            program[c] = program[a] + program[b]
        elif program[pc] == MUL:
            program[c] = program[a] * program[b]
        else:
            raise ValueError

        pc += 4

    return program


def main() -> None:
    with open("input.txt") as f:
        program = list(map(int, f.read().split(",")))

    program[1] = 12
    program[2] = 2

    intcode(program)

    print(program[0])


if __name__ == "__main__":
    main()

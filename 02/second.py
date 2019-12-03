#!/usr/bin/env python3
from itertools import product
from functools import partial
import typing as t

ADD = 1
MUL = 2
HALT = 99

EXPECTED = 19690720


def intcode(program: t.List[int]) -> t.List[int]:
    pc = 0

    while program[pc] != HALT:
        a, b, c = program[pc + 1 : pc + 4]

        if program[pc] == ADD:
            program[c] = program[a] + program[b]
        elif program[pc] == MUL:
            program[c] = program[a] * program[b]

        pc += 4

    return program


def execute(program: t.List[int], inp: t.Tuple[int, int]) -> t.Optional[int]:
    noun, verb = inp

    program = program.copy()
    program[1] = noun
    program[2] = verb

    if intcode(program)[0] == EXPECTED:
        return 100 * noun + verb
    else:
        return None


def main() -> None:
    with open("input.txt") as f:
        program = list(map(int, f.read().split(",")))

    print(
        next(
            filter(
                None,
                map(partial(execute, program), product(range(100), repeat=2)),
            )
        )
    )


if __name__ == "__main__":
    main()

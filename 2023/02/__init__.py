"""https://adventofcode.com/2023/day/2"""

from dataclasses import dataclass
from os import path
from typing import Optional

EX1 = """Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"""


# ---------------------------------- PART 1 ---------------------------------- #


@dataclass
class Game:
    id: int
    red: int
    blue: int
    green: int

    @staticmethod
    def parse(inp: str):
        [head, tail] = inp.split(":")


def part1(inp: str):
    return


# ---------------------------------------------------------------------------- #


# ---------------------------------- PART 2 ---------------------------------- #


def part2(inp: str):
    return


# ---------------------------------------------------------------------------- #


if __name__ == "__main__":
    print("Part 1:")
    print("  EX:", part1(EX1))
    with open(path.join(path.dirname(__file__), "in.txt")) as f:
        print(" ", part1(f.read()))

    # print("Part 2:")
    # print("  EX:", part2(EX2))
    # with open(path.join(path.dirname(__file__), "in.txt")) as f:
    #     print(" ", part2(f.read()))

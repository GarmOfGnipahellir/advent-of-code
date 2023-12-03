"""https://adventofcode.com/2023/day/3"""

from dataclasses import dataclass
from os import path
from typing import Optional, List

EX1 = """467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."""

ADJ = [
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
]


@dataclass
class Map:
    width: int
    height: int
    data: str

    def __iter__(self):
        return MapIterator(self)

    @staticmethod
    def parse(inp: str):
        lines = inp.splitlines()
        width = len(lines[0])
        height = len(lines)
        data = inp.replace("\n", "")
        return Map(width, height, data)

    def get(self, x: int, y: int):
        if x < 0 or x >= self.width or y < 0 or y >= self.height:
            return "."
        return self.data[x + y * self.width]


class MapIterator:
    def __init__(self, map: Map):
        self._map = map
        self._index = 0

    def __iter__(self):
        return self

    def __next__(self):
        if self._index < len(self._map.data):
            x = self._index % self._map.width
            y = self._index // self._map.width
            ch = self._map.get(x, y)
            self._index += 1
            return (x, y, ch)
        raise StopIteration


# ---------------------------------- PART 1 ---------------------------------- #


def part1(inp: str):
    map = Map.parse(inp)

    parts = []
    for y in range(map.height):
        numbuf = ""
        ispart = False
        for x in range(map.width):
            ch = map.get(x, y)
            if not ch.isdigit():
                if len(numbuf) > 0 and ispart:
                    parts.append(int(numbuf))
                numbuf = ""
                ispart = False
                continue

            numbuf += ch
            for ax, ay in ADJ:
                ach = map.get(x + ax, y + ay)
                if ach.isdigit() or ach == ".":
                    continue
                ispart = True

        if len(numbuf) > 0 and ispart:
            parts.append(int(numbuf))
    return sum(parts)


# ---------------------------------------------------------------------------- #


# ---------------------------------- PART 2 ---------------------------------- #


def part2(inp: str):
    map = Map.parse(inp)

    parts = []
    for y in range(map.height):
        numbuf = ""
        maybegear = False
        pos = (-1, -1)
        for x in range(map.width):
            ch = map.get(x, y)
            if not ch.isdigit():
                if len(numbuf) > 0 and maybegear:
                    parts.append((pos, int(numbuf)))
                numbuf = ""
                maybegear = False
                continue

            numbuf += ch
            for ax, ay in ADJ:
                ach = map.get(x + ax, y + ay)
                if ach != "*":
                    continue
                maybegear = True
                pos = (x + ax, y + ay)

        if len(numbuf) > 0 and maybegear:
            parts.append((pos, int(numbuf)))

    ratios = []
    done = []
    for i1, ((x1, y1), p1) in enumerate(parts):
        for i2, ((x2, y2), p2) in enumerate(parts):
            if i1 == i2 or i2 in done:
                continue

            if x1 == x2 and y1 == y2:
                ratios.append(p1 * p2)
                done.append(i1)
    return sum(ratios)


# ---------------------------------------------------------------------------- #


if __name__ == "__main__":
    print("Part 1:")
    print("  EX:", part1(EX1))
    with open(path.join(path.dirname(__file__), "in.txt")) as f:
        print(" ", part1(f.read()))

    print("Part 2:")
    print("  EX:", part2(EX1))
    with open(path.join(path.dirname(__file__), "in.txt")) as f:
        print(" ", part2(f.read()))

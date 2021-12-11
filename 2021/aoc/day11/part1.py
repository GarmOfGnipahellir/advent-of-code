# Advent of Code - Day 11 - Part One

from colorama import Fore, Style
from typing import List, Tuple, Union
from dataclasses import dataclass


@dataclass
class Octopus:
    energy: int
    flashed: bool

    def __str__(self) -> str:
        color = Fore.GREEN if self.flashed else Fore.RED
        return f"{color}{self.energy}{Style.RESET_ALL}"


class OctopusMap:
    width: int
    height: int
    rows: List[List[Octopus]]
    flashes: int

    def __init__(self, input: List[str]) -> None:
        self.width = len(input[0])
        self.height = len(input)
        self.rows = [[Octopus(int(h), False) for h in ln] for ln in input]
        self.flashes = 0

    def __eq__(self, other: object) -> bool:
        if type(self) != type(other) or self.width != other.width or self.height != other.height:
            return False
        return self.rows == other.rows

    def __str__(self) -> str:
        return "\n".join(["".join([str(octo) for octo in row]) for row in self.rows])

    def get(self, x: int, y: int) -> Union[Octopus, None]:
        if x < 0 or x >= self.width or y < 0 or y >= self.height:
            return None
        return self.rows[y][x]

    def get_adjacent(self, x: int, y: int) -> List[Tuple[int, int]]:
        return list(filter(lambda p: self.get(p[0], p[1]) != None, [
            (x, y + 1),
            (x + 1, y + 1),
            (x + 1, y),
            (x + 1, y - 1),
            (x, y - 1),
            (x - 1, y - 1),
            (x - 1, y),
            (x - 1, y + 1),
        ]))

    def flash(self, x: int, y: int) -> None:
        self.get(x, y).flashed = True
        self.flashes += 1
        for p in self.get_adjacent(x, y):
            octo = self.get(p[0], p[1])
            octo.energy += 1
            if octo.energy > 9 and not octo.flashed:
                self.flash(p[0], p[1])

    def step(self) -> None:
        # increase energy
        for y in range(self.height):
            for x in range(self.width):
                octo = self.get(x, y)
                octo.energy += 1

        # resolve flashing
        for y in range(self.height):
            for x in range(self.width):
                octo = self.get(x, y)
                if octo.energy > 9 and not octo.flashed:
                    self.flash(x, y)

        # reset energy
        for y in range(self.height):
            for x in range(self.width):
                octo = self.get(x, y)
                if octo.flashed:
                    octo.energy = 0

        # reset flashed
        for y in range(self.height):
            for x in range(self.width):
                octo = self.get(x, y)
                if octo.flashed:
                    octo.flashed = False


def result(input):
    octomap = OctopusMap(input)

    for i in range(100):
        octomap.step()

    return octomap.flashes

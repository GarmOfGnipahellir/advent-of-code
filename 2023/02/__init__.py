"""https://adventofcode.com/2023/day/2"""

from dataclasses import dataclass
from os import path
from typing import Optional, List
import re

EX1 = """Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"""

HAND_PAT = re.compile(r"(\d+) ((?:red)|(?:green)|(?:blue))")


@dataclass
class Bag:
    red: int
    green: int
    blue: int

    def power(self):
        return self.red * self.green * self.blue


@dataclass
class Hand:
    red: int = 0
    green: int = 0
    blue: int = 0

    @staticmethod
    def parse(inp: str):
        args = {}
        for m in HAND_PAT.finditer(inp):
            num = int(m.group(1))
            col = m.group(2)
            args[col] = num
        return Hand(**args)

    def ispossible(self, bag: Bag):
        return self.red <= bag.red and self.green <= bag.green and self.blue <= bag.blue


@dataclass
class Game:
    id: int
    hands: List[Hand]

    @staticmethod
    def parse(inp: str):
        [head, tail] = inp.split(":")
        id = int(head.replace("Game ", ""))
        hands = [Hand.parse(inp) for inp in tail.split(";")]
        return Game(id, hands)

    def ispossible(self, bag: Bag):
        for hand in self.hands:
            if not hand.ispossible(bag):
                return False
        return True

    def minbag(self):
        bag = Bag(0, 0, 0)
        for hand in self.hands:
            if hand.red > bag.red:
                bag.red = hand.red
            if hand.green > bag.green:
                bag.green = hand.green
            if hand.blue > bag.blue:
                bag.blue = hand.blue
        return bag


# ---------------------------------- PART 1 ---------------------------------- #


def part1(inp: str):
    bag = Bag(red=12, green=13, blue=14)
    sum = 0
    for line in inp.splitlines():
        game = Game.parse(line)
        if game.ispossible(bag):
            sum += game.id
    return sum


# ---------------------------------------------------------------------------- #


# ---------------------------------- PART 2 ---------------------------------- #


def part2(inp: str):
    sum = 0
    for line in inp.splitlines():
        game = Game.parse(line)
        bag = game.minbag()
        sum += bag.power()
    return sum


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

from collections import namedtuple
from enum import Enum
from dataclasses import dataclass

inp = open("./in00", mode="r").readlines()
inp = [line.strip() for line in inp]


Instruction = namedtuple("Instruction", ["cmd", "val"])
Point = namedtuple("Point", ["x", "y"])


class Direction(Enum):
    N = 0
    E = 1
    S = 2
    W = 3

    def to_point(self):
        if self.value == 0:
            return Point(0, -1)
        elif self.value == 1:
            return Point(1, 0)
        elif self.value == 2:
            return Point(0, 1)
        elif self.value == 3:
            return Point(-1, 0)


@dataclass
class Ship:
    facing: Direction
    position: Point

    def execute_instruction(self, instruction: Instruction):
        (cmd, val) = instruction
        delta: Point = Point(0, 0)
        if cmd == "N":
            delta = Point(0, -1)
        elif cmd == "E":
            delta = Point(1, 0)
        elif cmd == "S":
            delta = Point(0, 1)
        elif cmd == "W":
            delta = Point(-1, 0)
        elif cmd == "L":
            self.facing = Direction((self.facing.value - (val // 90)) % 4)
        elif cmd == "R":
            self.facing = Direction((self.facing.value + (val // 90)) % 4)
        elif cmd == "F":
            delta = self.facing.to_point()

        self.position = Point(
            self.position.x + delta.x * val,
            self.position.y + delta.y * val,
        )


instructions = [Instruction(ln[:1], int(ln[1:])) for ln in inp]

ship = Ship(Direction.E, Point(0, 0))

print(ship)
for instruction in instructions:
    print(instruction)
    ship.execute_instruction(instruction)
    print(ship)

print(abs(ship.position.x) + abs(ship.position.y))
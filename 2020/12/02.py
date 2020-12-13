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
    waypoint: Point

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
            if val == 90:
                self.waypoint = Point(
                    self.waypoint.y,
                    -self.waypoint.x,
                )
            elif val == 180:
                self.waypoint = Point(
                    -self.waypoint.x,
                    -self.waypoint.y,
                )
            elif val == 270:
                self.waypoint = Point(
                    -self.waypoint.y,
                    self.waypoint.x,
                )
        elif cmd == "R":
            if val == 90:
                self.waypoint = Point(
                    -self.waypoint.y,
                    self.waypoint.x,
                )
            elif val == 180:
                self.waypoint = Point(
                    -self.waypoint.x,
                    -self.waypoint.y,
                )
            elif val == 270:
                self.waypoint = Point(
                    self.waypoint.y,
                    -self.waypoint.x,
                )
        elif cmd == "F":
            self.position = Point(
                self.position.x + self.waypoint.x * val,
                self.position.y + self.waypoint.y * val,
            )
        
        self.waypoint = Point(
            self.waypoint.x + delta.x * val,
            self.waypoint.y + delta.y * val,
        )

instructions = [Instruction(ln[:1], int(ln[1:])) for ln in inp]

ship = Ship(Direction.E, Point(0, 0), Point(10, -1))

print(ship)
for instruction in instructions:
    print(instruction)
    ship.execute_instruction(instruction)
    print(ship)

print(abs(ship.position.x) + abs(ship.position.y))

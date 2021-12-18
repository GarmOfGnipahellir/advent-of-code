# Advent of Code - Day 17 - Part One

import re
from dataclasses import dataclass
from typing_extensions import Self

AREA_PAT = re.compile(r"x=(.+?)\.\.(.+?), y=(.+?)\.\.(.+?)$")


@dataclass
class Vec:
    x: int
    y: int

    def __add__(self, other: object) -> Self:
        if type(self) != type(other):
            raise ValueError(f"cant add Vec with {type(other)}")
        return Vec(self.x + other.x, self.y + other.y)


@dataclass
class Area:
    min: Vec
    max: Vec

    def parse(input: str) -> Self:
        match = re.search(AREA_PAT, input)
        return Area(
            Vec(
                int(match.group(1)),
                int(match.group(3)),
            ),
            Vec(
                int(match.group(2)),
                int(match.group(4)),
            ),
        )


# x = v*x - 0.5 * max(x-0.5, 0)^2
# y = v*x - 0.5 * (x-0.5)^2
@dataclass
class Probe:
    pos: Vec
    vel: Vec

    def step(self) -> None:
        self.pos += self.vel
        if self.vel.x > 0:
            self.vel.x -= 1
        elif self.vel.x < 0:
            self.vel.x += 1
        self.vel.y -= 1


def result(input):
    area = Area.parse(input[0])
    print(area)
    probe = Probe(Vec(0, 0), Vec(6, 3))
    print(probe)
    for _ in range(7):
        probe.step()
        print(probe)
    return 45

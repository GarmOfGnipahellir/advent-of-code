# Advent of Code - Day 17 - Part One

import re
import math
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


@dataclass
class Trajectory:
    initial_vel: Vec

    def max_x_time(self) -> int:
        return abs(self.initial_vel.x)

    def time_at_y(self, y) -> float:
        v2 = self.initial_vel.y * 2
        return 0.5 * (math.sqrt((v2 + 1) ** 2 - 8 * y) + v2 + 1)

    def evaluate(self, time: int) -> Vec:
        xt = min(time, self.max_x_time())
        return Vec(
            ((self.initial_vel.x * 2 + 1) * xt - xt ** 2) / 2,
            ((self.initial_vel.y * 2 + 1) * time - time ** 2) / 2,
        )


def result(input):
    area = Area.parse(input[0])
    record_vy = 0
    record_t = 0
    t = 0
    for vy in range(0, abs(area.max.y) * 10):
        hit = False
        trajectory = Trajectory(Vec(0, vy))
        for y in reversed(range(area.min.y, area.max.y + 1)):
            t = trajectory.time_at_y(y)
            if round(t) == t:
                hit = True
                break

        if hit:
            record_vy = vy
            record_t = t

        vy += 1

    return Trajectory(Vec(0, record_vy)).evaluate(record_t // 2).y

# Advent of Code - Day 17 - Part One

import re
from dataclasses import dataclass
from typing_extensions import Self

AREA_PAT = re.compile(r"x=(.+?)\.\.(.+?), y=(.+?)\.\.(.+?)$")


@dataclass
class Area:
    xmin: int
    xmax: int
    ymin: int
    ymax: int

    def parse(input: str) -> Self:
        match = re.search(AREA_PAT, input)
        return Area(
            int(match.group(1)),
            int(match.group(2)),
            int(match.group(3)),
            int(match.group(4)),
        )


def result(input):
    area = Area.parse(input[0])
    print(area)
    return 45
    
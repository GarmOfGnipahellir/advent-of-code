# Advent of Code - Day 13 - Part Two

from dataclasses import dataclass
from typing import List


@dataclass
class Dot:
    x: int
    y: int


@dataclass
class Fold:
    dir: str
    pos: int


@dataclass
class Paper:
    dots: List[Dot]

    def __str__(self) -> str:
        w = max([d.x for d in self.dots]) + 1
        h = max([d.y for d in self.dots]) + 1

        lines = []
        for _ in range(h):
            line = []
            for _ in range(w):
                line.append(".")
            lines.append(line)

        for d in self.dots:
            lines[d.y][d.x] = "#"

        return "\n".join(["".join(line) for line in lines])

    def fold(self, f: Fold) -> None:
        newdots = []
        for d in self.dots:
            if f.dir == "x" and d.x > f.pos:
                delta = d.x - f.pos
                d.x -= delta * 2
            elif f.dir == "y" and d.y > f.pos:
                delta = d.y - f.pos
                d.y -= delta * 2

            if d not in newdots:
                newdots.append(d)
        self.dots = newdots


def result(input):
    dots = []
    i = 0
    while input[i] != "":
        dots.append(Dot(*[int(n) for n in input[i].split(",")]))
        i += 1

    folds = []
    i += 1
    while i < len(input):
        dir, pos = input[i][11:].split("=")
        folds.append(Fold(dir, int(pos)))
        i += 1

    paper = Paper(dots)
    for f in folds:
        paper.fold(f)

    return "\n" + str(paper)

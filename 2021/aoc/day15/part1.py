# Advent of Code - Day 15 - Part One

from colorama import Fore, Style
from dataclasses import dataclass, field
from typing import Any, List, Tuple, Union
from queue import PriorityQueue
from typing_extensions import Self


@dataclass
class Point:
    x: int
    y: int

    def __hash__(self) -> int:
        return hash((self.x, self.y))

    def adjacent(self) -> List[Self]:
        return [
            Point(self.x, self.y + 1),
            Point(self.x + 1, self.y),
            Point(self.x, self.y - 1),
            Point(self.x - 1, self.y),
        ]


@dataclass(order=True)
class PrioItem:
    prio: int
    item: Any = field(compare=False)


class Riskmap:
    width: int
    height: int
    levels: List[int]

    def __init__(self, input: List[str]) -> None:
        self.height = len(input)
        self.width = len(input[0])
        self.levels = [int(l) for l in "".join(input)]

    def __str__(self) -> str:
        lines = []
        for y in range(self.height):
            line = ""
            for x in range(self.width):
                line += str(self.get(Point(x, y)))
            lines.append(line)
        return "\n".join(lines)

    def vis_path(self, path: List[Point]) -> str:
        lines = []
        for y in range(self.height):
            line = ""
            for x in range(self.width):
                if Point(x, y) in path:
                    line += f"{Fore.GREEN}{self.get(Point(x, y))}{Style.RESET_ALL}"
                else:
                    line += str(self.get(Point(x, y)))
            lines.append(line)
        return "\n".join(lines)

    def get(self, p: Point) -> Union[int, None]:
        if p.x < 0 or p.x >= self.width or p.y < 0 or p.y >= self.height:
            return None
        return self.levels[p.x + p.y * self.width]

    def adjacent(self, p: Point) -> List[Point]:
        return list(filter(lambda o: self.get(o) != None, p.adjacent()))

    def dijkstra(self, start: Point, end: Point) -> List[Point]:
        frontier = PriorityQueue()
        frontier.put(PrioItem(0, start))
        camefrom = {start: None}
        risks = {start: 0}

        while not frontier.empty():
            current = frontier.get().item

            if current == end:
                break

            for next in self.adjacent(current):
                risk = risks[current] + self.get(next)
                if next not in risks or risk < risks[next]:
                    risks[next] = risk
                    frontier.put(PrioItem(risk, next))
                    camefrom[next] = current

        path = []
        current = end
        while current != start:
            path.append(current)
            current = camefrom[current]
        return path


def result(input):
    riskmap = Riskmap(input)
    path = riskmap.dijkstra(Point(0, 0), Point(
        riskmap.width-1, riskmap.height-1))
    risk = sum([riskmap.get(p) for p in path])
    return risk

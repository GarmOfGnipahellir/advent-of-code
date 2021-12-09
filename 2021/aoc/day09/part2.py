# Advent of Code - Day 9 - Part Two

from typing import List, Tuple, Union


class Heightmap:
    width: int
    height: int
    rows: List[List[int]]

    def __init__(self, input: List[str]) -> None:
        self.width = len(input[0])
        self.height = len(input)
        self.rows = [[int(h) for h in ln] for ln in input]

    def get(self, x: int, y: int) -> Union[int, None]:
        if x < 0 or x >= self.width or y < 0 or y >= self.height:
            return None
        return self.rows[y][x]

    def get_adjacent(self, x: int, y: int) -> List[int]:
        return list(filter(lambda i: i != None, [
            self.get(x - 1, y),
            self.get(x + 1, y),
            self.get(x, y - 1),
            self.get(x, y + 1),
        ]))


def result(input):
    heightmap = Heightmap(input)

    lowpoints = []
    for y in range(heightmap.height):
        for x in range(heightmap.width):
            center = heightmap.get(x, y)
            adjacent = heightmap.get_adjacent(x, y)

            islow = True
            for i in adjacent:
                if center >= i:
                    islow = False

            if islow:
                lowpoints.append((x, y))

    basins = []
    for lowpoint in lowpoints:
        basin = []
        frontier = [lowpoint]
        i = 0
        while len(frontier) > 0 and i < len(frontier):
            x, y = frontier[i]
            basin.append((x, y))
            adj = [
                (x - 1, y),
                (x + 1, y),
                (x, y - 1),
                (x, y + 1),
            ]
            for pt in adj:
                h = heightmap.get(pt[0], pt[1])
                if h != None and h != 9 and pt not in basin and pt not in frontier:
                    frontier.append(pt)
            i += 1
        basins.append(len(basin))

    basins.sort()
    res = basins[-3]
    for basin in basins[-2:]:
        res *= basin
    return res

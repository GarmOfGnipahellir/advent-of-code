# Advent of Code - Day 9 - Part One

from typing import List, Union


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
                lowpoints.append(center)

    return sum([i + 1 for i in lowpoints])

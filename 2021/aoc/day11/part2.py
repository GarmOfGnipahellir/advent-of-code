# Advent of Code - Day 11 - Part Two

from .part1 import OctopusMap


def result(input):
    octomap = OctopusMap(input)
    i = 1
    while True:
        octomap.step()
        allflashed = True
        for y in range(octomap.height):
            for x in range(octomap.width):
                if octomap.get(x, y).energy != 0:
                    allflashed = False
        if allflashed:
            return i
        i += 1

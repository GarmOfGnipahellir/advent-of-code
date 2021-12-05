# Advent of Code - Day 5 - Part One

from typing import List


def result(input: List[str]):
    points = {}
    for ln in input:
        start, end = [(*[int(val) for val in pt.split(",")],)
                      for pt in ln.split(" -> ")]
        if start[0] == end[0]:
            mn = min(start[1], end[1])
            mx = max(start[1], end[1])
            for i in range(mn, mx + 1):
                pt = (start[0], i)
                if pt not in points:
                    points[pt] = 1
                else:
                    points[pt] += 1
        if start[1] == end[1]:
            mn = min(start[0], end[0])
            mx = max(start[0], end[0])
            for i in range(mn, mx + 1):
                pt = (i, start[1])
                if pt not in points:
                    points[pt] = 1
                else:
                    points[pt] += 1

    return len(list(filter(lambda x: x > 1, points.values())))

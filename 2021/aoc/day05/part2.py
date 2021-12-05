# Advent of Code - Day 5 - Part Two

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
        elif start[1] == end[1]:
            mn = min(start[0], end[0])
            mx = max(start[0], end[0])
            for i in range(mn, mx + 1):
                pt = (i, start[1])
                if pt not in points:
                    points[pt] = 1
                else:
                    points[pt] += 1
        else:
            x = range(min(start[0], end[0]), max(start[0], end[0]) + 1)
            if start[0] > end[0]:
                x = reversed(x)
            y = range(min(start[1], end[1]), max(start[1], end[1]) + 1)
            if start[1] > end[1]:
                y = reversed(y)
            for pt in zip(x, y):
                if pt not in points:
                    points[pt] = 1
                else:
                    points[pt] += 1

    return len(list(filter(lambda x: x > 1, points.values())))

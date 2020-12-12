from enum import Enum
from os import path
__dir__ = path.dirname(__file__)

with open(f'{__dir__}/in', 'r') as f:
    turtles = [l.split(',') for l in f.readlines()]


class Axis(Enum):
    VERTICAL = 1
    HORIZONTAL = 2


class Line:
    def __init__(self, start, end, dir, dist, parent):
        self.parent = parent

        self.start = start
        self.end = end

        self.dist = dist

        self.min = [min(start[0], end[0]), min(start[1], end[1])]
        self.max = [max(start[0], end[0]), max(start[1], end[1])]

        if dir == 'U' or dir == 'D':
            self.axis = Axis.VERTICAL
        elif dir == 'R' or dir == 'L':
            self.axis = Axis.HORIZONTAL

    def intersect(self, other):
        if self.axis == other.axis:
            return None

        v = self if self.axis == Axis.VERTICAL else other
        h = self if self.axis == Axis.HORIZONTAL else other

        if (v.min[0] >= h.min[0] and v.max[0] <= h.max[0] and
                h.min[1] >= v.min[1] and h.max[1] <= v.max[1]):
            return [v.min[0], h.min[1]]

    def __str__(self):
        return f'{self.min} -> {self.max}: {self.axis}'


wires = [[], []]
for i, commands in enumerate(turtles):
    print(f'Wire {i}:')
    start = [0, 0]
    for j, command in enumerate(commands):
        command = command.strip()
        dir = command[0]
        dist = int(command[1::])

        if dir == 'L' or dir == 'D':
            dist = -dist

        end = start.copy()
        if dir == 'R' or dir == 'L':
            end[0] += dist
        elif dir == 'U' or dir == 'D':
            end[1] += dist

        parent = None
        if j > 0:
            parent = wires[i][j-1]

        line = Line(start, end, dir, abs(dist), parent)

        print(' ', command, line)
        wires[i].append(line)

        start = end

closest = 2**128
for line1 in wires[0]:
    for line2 in wires[1]:
        cross = line1.intersect(line2)
        if cross != None and cross != [0, 0]:
            dist1 = abs(cross[0] - line1.start[0] + cross[1] - line1.start[1])
            dist2 = abs(cross[0] - line2.start[0] + cross[1] - line2.start[1])
            while line1.parent != None:
                line1 = line1.parent
                dist1 += line1.dist
            while line2.parent != None:
                line2 = line2.parent
                dist2 += line2.dist
            
            if dist1 + dist2 < closest:
                closest = dist1 + dist2

print(closest)
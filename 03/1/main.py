from enum import Enum
from os import path
__dir__ = path.dirname(__file__)

with open(f'{__dir__}/ex.1', 'r') as f:
    turtles = [l.split(',') for l in f.readlines()]


class Axis(Enum):
    VERTICAL = 1
    HORIZONTAL = 2


class Line:
    def __init__(self, start, end, dir):
        self.start = start
        self.end = end

        if dir == 'U' or dir == 'D':
            self.axis = Axis.VERTICAL
        elif dir == 'R' or dir == 'L':
            self.axis = Axis.HORIZONTAL

    def intersect(self, other):
        if self.axis == other.axis:
            return None
        

    def __str__(self):
        return f'{self.start} -> {self.end}: {self.axis}'


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

        line = Line(start, end, dir)

        print(' ', command, line)
        wires[i].append(line)

        start = end

from math import sqrt, atan2, pi
from os import path
__dir__ = path.dirname(__file__)


def calc_angle(origin, target):
    delta = [
        target[0] - origin[0],
        target[1] - origin[1]
    ]

    if delta[0] == 0:
        if delta[1] > 0:
            angle = pi
        else:
            angle = 0
    elif delta[1] == 0:
        if delta[0] > 0:
            angle = pi / 2
        else:
            angle = pi + (pi / 2)
    else:
        angle = atan2(delta[0], -delta[1])
        if delta[0] < 0 and delta[1] > 0:
            angle = abs(angle - (pi / 2))
        elif delta[0] < 0 and delta[1] < 0:
            angle = angle + (pi * 2)
    
    return angle

class AsteroidMap:
    def __init__(self, lines):
        self.set_data([list(l.strip()) for l in lines])

    def set_data(self, data):
        self.data = data
        self.asteroids = []
        for y, row in enumerate(self.data):
            for x, value in enumerate(row):
                if value == '#':
                    self.asteroids.append([x, y])

    @staticmethod
    def print_data(data):
        print('\n'.join([''.join([str(y) for y in x]) for x in data]))

    @staticmethod
    def combine_los_maps(los_maps):
        data = [[1 for _ in row] for y, row in enumerate(los_maps[0])]
        for los_map in los_maps:
            for y, row in enumerate(los_map):
                for x, value in enumerate(row):
                    data[y][x] *= value
        return data

    def get(self, x, y):
        return self.data[y][x]

    def los_map(self, origin, blocker):
        data = [[1 for _ in row] for y, row in enumerate(self.data)]

        block_delta = [
            blocker[0] - origin[0],
            blocker[1] - origin[1]
        ]

        block_dist = sqrt(block_delta[0]**2 + block_delta[1]**2)

        for y, row in enumerate(self.data):
            for x, value in enumerate(row):
                delta = [x - origin[0], y - origin[1]]
                dist = sqrt(delta[0]**2 + delta[1]**2)

                if dist > block_dist:
                    dot = (
                        ((block_delta[0] / block_dist) * (delta[0] / dist)) +
                        ((block_delta[1] / block_dist) * (delta[1] / dist))
                    )
                    if dot > 0.999999:
                        data[y][x] = 0

        return data

    def combined_los_map(self, origin):
        los_maps = []
        for blocker in self.asteroids:
            if origin == blocker:
                continue
            los_maps.append(self.los_map(origin, blocker))
        return AsteroidMap.combine_los_maps(los_maps)

    def remove_los_blocked(self, origin, los_map):
        data = [['.' for _ in row] for y, row in enumerate(self.data)]
        for y, row in enumerate(los_map):
            for x, value in enumerate(row):
                if value == 1 and self.get(x, y) == '#' and origin != [x, y]:
                    data[y][x] = '#'
        return data

    def visible_map(self, origin):
        los_map = self.combined_los_map(pos)
        return self.remove_los_blocked(pos, los_map)

    def order_by_angle(self, pos):
        result = self.asteroids.copy()
        result.sort(key=lambda x: calc_angle(pos, x))
        for i, asteroid in enumerate(result):
            if i < 9:
                self.data[asteroid[1]][asteroid[0]] = i+1
        return result

    def print(self):
        AsteroidMap.print_data(self.data)


with open(f'{__dir__}/in', 'r') as f:
    lines = f.readlines()
    pos = [int(lines[0].strip()), int(lines[1].strip())]
    asteroid_map = AsteroidMap(lines[2::])
    asteroid_map.print()
    print()
    asteroid_map.set_data(asteroid_map.visible_map(pos))
    asteroid_map.print()
    print()
    ordered = asteroid_map.order_by_angle(pos)
    asteroid_map.print()
    print()
    answer = ordered[199]
    print(answer)
    print(answer[0] * 100 + answer[1])
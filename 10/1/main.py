from math import floor, copysign
from os import path
__dir__ = path.dirname(__file__)


class AsteroidMap:
    def __init__(self, lines):
        self.data = [list(l.strip()) for l in lines]

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
        height = len(data)
        width = len(data[0])

        delta = [blocker[0] - origin[0], blocker[1] - origin[1]]

        new_delta = [delta[0], delta[1]]
        if delta[1] != 0:
            ratio = abs(delta[0] / delta[1])
            if ratio - floor(ratio) == 0 and ratio != 0:
                new_delta = [int(ratio), 1]
        elif delta[0] != 0:
            ratio = abs(delta[1] / delta[0])
            if ratio - floor(ratio) == 0 and ratio != 0:
                new_delta = [1, int(ratio)]
        
        if delta[1] == 0:
            new_delta[0] = 1
        elif delta[0] == 0: 
            new_delta[1] = 1
        
        new_delta = [
            int(copysign(new_delta[0], delta[0])), 
            int(copysign(new_delta[1], delta[1])),
        ]
        delta = new_delta

        pos = [blocker[0] + delta[0], blocker[1] + delta[1]]
        while pos[0] < width and pos[1] < height and pos[0] >= 0 and pos[1] >= 0:
            data[pos[1]][pos[0]] = 0
            pos = [pos[0] + delta[0], pos[1] + delta[1]]

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

    def print(self):
        AsteroidMap.print_data(self.data)


with open(f'{__dir__}/ex.4', 'r') as f:
    asteroid_map = AsteroidMap(f.readlines())
    asteroid_map.print()

    record = 0
    record_pos = [-1, -1]
    for asteroid in asteroid_map.asteroids:
        print()
        los_map = asteroid_map.combined_los_map(asteroid)
        los_blocked = asteroid_map.remove_los_blocked(asteroid, los_map)
        AsteroidMap.print_data(los_blocked)

        count = 0
        for y, row in enumerate(los_blocked):
            for x, value in enumerate(row):
                if value == '#':
                    count += 1
        print(asteroid)
        print(count)

        if count > record:
            record = count
            record_pos = asteroid
    
    print()
    print(record_pos)
    print(record)

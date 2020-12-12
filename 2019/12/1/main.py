from os import path
__dir__ = path.dirname(__file__)


def sign(num):
    if num > 0:
        return 1
    elif num < 0:
        return -1
    else:
        return 0


class Moon:
    def __init__(self, posstr):
        self.pos = tuple(
            [int(x[2::]) for x in posstr[1:-1].split(', ')])
        self.vel = (0, 0, 0)

    def __str__(self):
        return f'<pos={self.pos}, vel={self.vel}>'

    def apply_gravity(self, other):
        vel = list(self.vel)
        for i, (val1, val2) in enumerate(zip(self.pos, other.pos)):
            vel[i] += sign(val2 - val1)
        self.vel = tuple(vel)

    def apply_velocity(self):
        pos = list(self.pos)
        for i, val in enumerate(self.vel):
            pos[i] += val
        self.pos = tuple(pos)

    def get_energy(self):
        pot = abs(self.pos[0]) + abs(self.pos[1]) + abs(self.pos[2])
        kin = abs(self.vel[0]) + abs(self.vel[1]) + abs(self.vel[2])
        return pot * kin


with open(f'{__dir__}/in', 'r') as f:
    moons = [Moon(line.strip()) for line in f.readlines()]

print('Iteration 0')
for i, moon in enumerate(moons):
    print(' ', i, moon)

for iteration in range(1000):
    print('Iteration', iteration+1)

    for moon in moons:
        for other in moons:
            if moon == other:
                continue
            moon.apply_gravity(other)

    for moon in moons:
        moon.apply_velocity()

    for i, moon in enumerate(moons):
        print(' ', i, moon)

print('Energy')
total = 0
for i, moon in enumerate(moons):
    energy = moon.get_energy()
    total += energy
    print('   ', i, energy)

print('  tot', total)

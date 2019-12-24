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

    def apply_gravity(self, other, dim):
        vel = list(self.vel)
        vel[dim] += sign(other.pos[dim] - self.pos[dim])
        self.vel = tuple(vel)

    def apply_velocity(self, dim):
        pos = list(self.pos)
        pos[dim] += self.vel[dim]
        self.pos = tuple(pos)

    def get_energy(self):
        pot = abs(self.pos[0]) + abs(self.pos[1]) + abs(self.pos[2])
        kin = abs(self.vel[0]) + abs(self.vel[1]) + abs(self.vel[2])
        return pot * kin


with open(f'{__dir__}/in', 'r') as f:
    moons = [Moon(line.strip()) for line in f.readlines()]

iterations = [-1, -2, -3]
iteration = 1
while True:
    for dim in range(3):
        for moon in moons:
            for other in moons:
                if moon == other:
                    continue
                moon.apply_gravity(other, dim)

        for moon in moons:
            moon.apply_velocity(dim)

        all_vels_zero = True
        for moon in moons:
            if moon.vel[dim] != 0:
                all_vels_zero = False
        
        if all_vels_zero:
            iterations[dim] = iteration
    
    all_equal = True
    for val1 in iterations:
        for val2 in iterations:
            if val1 != val2:
                all_equal = False
    
    if all_equal:
        print(iteration)
        break

    if iteration % 100000 == 0:
        print(iteration)
    
    iteration += 1

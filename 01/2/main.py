from os import path
from math import floor

__dir__ = path.dirname(__file__)


def fuel_for_mass(m):
    return max(0, floor(m / 3) - 2)


def fuel_for_module(m):
    result = 0
    f = fuel_for_mass(m)
    while f > 0:
        result += f
        f = fuel_for_mass(f)
    return result


result = 0
with open(f'{__dir__}/in', 'r') as f:
    for l in f.readlines():
        m = int(l.strip())
        f = fuel_for_module(m)
        result += f
print(result)

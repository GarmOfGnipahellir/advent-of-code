from os import path
from math import floor

__dir__ = path.dirname(__file__)

result = 0
with open(f'{__dir__}/in', 'r') as f:
    for l in f.readlines():
        m = int(l.strip())
        f = floor(m / 3) - 2
        result += f
print(result)

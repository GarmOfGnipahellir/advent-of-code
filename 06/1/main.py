from os import path
__dir__ = path.dirname(__file__)

with open(f'{__dir__}/in', 'r') as f:
    orbits = [tuple(line.strip().split(')')) for line in f.readlines()]


class Body:
    def __init__(self, name):
        self.name = name
        self.parent = None

    def __str__(self):
        return f'Body({self.name})'


names = []
for (parent, child) in orbits:
    if parent not in names:
        names.append(parent)
    if child not in names:
        names.append(child)

bodies = {}
for name in names:
    bodies[name] = Body(name)

for (parent, child) in orbits:
    bodies[child].parent = bodies[parent]

total = 0
for name, body in bodies.items():
    count = 0
    path = body
    while path.name != 'COM':
        count += 1
        path = path.parent
    total += count

print(total)

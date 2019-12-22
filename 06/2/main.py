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

paths = {
    'YOU': [],
    'SAN': [],
}
for who in paths:
    path = bodies[who].parent
    while path.name != 'COM':
        paths[who].append(path)
        path = path.parent

common_body = None
for body1 in paths['YOU']:
    for body2 in paths['SAN']:
        if body1 == body2:
            common_body = body1
            break
    if common_body != None:
        break

jumps = 0
for who in paths:
    path = bodies[who].parent
    while path != common_body:
        jumps += 1
        path = path.parent

print(jumps)
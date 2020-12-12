inp = open("./in01", mode="r").readlines()
inp = [line.strip() for line in inp]

slopes = [
    (1, 1),
    (3, 1),
    (5, 1),
    (7, 1),
    (1, 2),
]

prevn = 1
for slope in slopes:
    pos = (0, 0)

    n = 0
    while pos[1] != len(inp) - 1:
        pos = ((pos[0] + slope[0]) % len(inp[0]), pos[1] + slope[1])
        if inp[pos[1]][pos[0]] == "#":
            n += 1
    print(n)
    prevn *= n

print(prevn)
inp = open("./in01", mode="r").readlines()
inp = [int(line) for line in inp]

def find_answer():
    for i, n in enumerate(inp):
        for j, m in enumerate(inp):
            if i == j: continue

            if n + m == 2020:
                return n * m
    return -1

print(find_answer())

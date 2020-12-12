inp = open("./in01", mode="r").readlines()
inp = [int(line) for line in inp]

def find_answer():
    for i, n in enumerate(inp):
        for j, m in enumerate(inp):
            if j == i: continue
            for k, o in enumerate(inp):
                if k == j: continue

                if n + m + o == 2020:
                    return n * m * o
    return -1

print(find_answer())

inp = open("./in00", mode="r").readlines()
inp = [int(line.strip()) for line in inp]

print(inp)

preamble = 25

for i, n in enumerate(inp[preamble:]):
    to_check = inp[i:i + preamble]
    print(n, to_check)
    nvalid = 0
    for j, m in enumerate(to_check):
        for k, o in enumerate(to_check):
            if i == j: continue

            if m + o == n:
                nvalid += 1
    
    if nvalid == 0:
        print(n)
        break
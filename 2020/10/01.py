inp = open("./in00", mode="r").readlines()
inp = [int(line.strip()) for line in inp]

chain = inp.copy()
chain.append(0)
chain.append(max(chain) + 3)
chain.sort()

print(chain)

n1diffs = 0
n3diffs = 0
for i in range(1, len(chain)):
    n1 = chain[i-1]
    n2 = chain[i]
    diff = n2 - n1
    print(n1, n2, diff)
    if diff == 1:
        n1diffs += 1
    elif diff == 3:
        n3diffs += 1

print(n1diffs, n3diffs)
print(n1diffs * n3diffs)
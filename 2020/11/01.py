import copy

inp = open("./in00", mode="r").readlines()
inp = [list(line.strip()) for line in inp]

slope = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),

    (1, 1),
    (-1, 1),
    (-1, -1),
    (1, -1),
]

preiter = copy.deepcopy(inp)
changed_seats = -1
while changed_seats != 0:
    changed_seats = 0

    nxtiter = copy.deepcopy(preiter)

    h = len(preiter)
    for i in range(h):
        ln = preiter[i]
        w = len(ln)
        for j in range(w):
            ch = ln[j]
            
            nocc = 0
            for s in slope:
                k = 0
                while True:
                    k += 1

                    y = i + s[1] * k
                    x = j + s[0] * k

                    if not (x >= 0 and x < w and y >= 0 and y < h):
                        break

                    if preiter[y][x] == "#":
                        nocc += 1
                        break
                    elif preiter[y][x] == "L":
                        break
            
            if nocc == 0 and preiter[i][j] == "L":
                nxtiter[i][j] = "#"
                changed_seats += 1
            
            if nocc >= 5 and preiter[i][j] == "#":
                nxtiter[i][j] = "L"
                changed_seats += 1

    for ln in nxtiter:
        print("".join(ln))
    print()
    
    preiter = copy.deepcopy(nxtiter)

ans = 0
for ln in preiter:
    for ch in ln:
        if ch == "#":
            ans += 1

print(ans)
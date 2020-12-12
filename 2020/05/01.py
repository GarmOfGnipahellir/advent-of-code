inp = open("./in01", mode="r").readlines()
inp = [line.strip() for line in inp]

highest = 0
for boardingpass in inp:
    rows = list(range(128))
    for ch in boardingpass[:7]:
        if ch == "F":
            rows = rows[:len(rows)//2]
        elif ch == "B":
            rows = rows[len(rows)//2:]
    
    cols = list(range(8))
    for ch in boardingpass[7:]:
        if ch == "L":
            cols = cols[:len(cols)//2]
        elif ch == "R":
            cols = cols[len(cols)//2:]
    
    sid = rows[0] * 8 + cols[0]
    print(boardingpass, sid)

    if sid > highest:
        highest = sid

print(highest)
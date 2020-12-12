inp = open("./in01", mode="r").readlines()
inp = [line.strip() for line in inp]

grps = []
grp = []
for ln in inp:
    if ln == "":
        grps.append(grp)
        grp = []
        continue
    
    grp.append(ln)
grps.append(grp)

sm = 0
for grp in grps:
    ans = "".join(grp)
    uni = ""

    for ch in ans:
        if ch not in uni:
            uni += ch

    sm += len(uni)

print(sm)
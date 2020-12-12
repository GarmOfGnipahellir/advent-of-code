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
    ans = {}
    for pers in grp:
        for ch in pers:
            if ch not in ans.keys():
                ans[ch] = 1
            else:
                ans[ch] += 1
    
    for v in ans.values():
        if v == len(grp):
            sm += 1

print(sm)
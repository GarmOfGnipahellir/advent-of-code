import re

inp = open("./in00", mode="r").readlines()
inp = [line.strip() for line in inp]

mask_pat = re.compile(r"mask = ([X01]+)")
mem_pat = re.compile(r"mem\[(\d*)\] = (\d*)")

match = mask_pat.match(inp[0])
mask = match.group(1)
print(len(mask), mask)

mem = []
for ln in inp:
    match = mask_pat.match(ln)
    if match is not None:
        mask = match.group(1)
        continue

    match = mem_pat.match(ln)
    pos = int(match.group(1))
    binval = "{0:b}".format(int(match.group(2))).zfill(len(mask))
    mskval = ""
    for i in range(len(mask)):
        if mask[i] == "X":
            mskval += binval[i]
        else:
            mskval += mask[i]
    decval = int(mskval, 2)
    print(pos, binval, mskval, decval)

    
    if pos >= len(mem):
        for i in range(pos - len(mem) + 1):
            mem.append(0)
    mem[pos] = decval
    print(mem)

print(sum(mem))
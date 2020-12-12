import re

inp = open("./in00", mode="r").readlines()
inp = [line.strip() for line in inp]

op_pat = re.compile(r"(\w{3}) ([-+]\d+)")

ops = []
for ln in inp:
    match = op_pat.match(ln)
    op = match.group(1)
    value = int(match.group(2))

    ops.append((op, value))

has_run = []
i = 0
acc = 0
while i not in has_run:
    has_run.append(i)

    op = ops[i]
    if op[0] == "nop":
        i += 1
    elif op[0] == "acc":
        acc += op[1]
        i += 1
    elif op[0] == "jmp":
        i += op[1]

print(acc)
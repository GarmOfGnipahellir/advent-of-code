import re

inp = open("./in01", mode="r").readlines()
inp = [line.strip() for line in inp]

parse_pattern = r"^(\d*)-(\d*) (\w): (\w+)"
def parse_line(line):
    match = re.match(parse_pattern, line)
    mn = int(match.group(1))
    mx = int(match.group(2))
    ch = match.group(3)
    st = match.group(4)
    return (mn, mx, ch, st)

n = 0
for line in inp:
    (mn, mx, ch, st) = parse_line(line)
    
    i = 0
    for x in st:
        if x == ch:
            i += 1
    
    if i <= mx and i >= mn:
        n += 1

print(n)
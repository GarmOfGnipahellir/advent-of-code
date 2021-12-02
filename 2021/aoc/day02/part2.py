# Advent of Code - Day 2 - Part Two

def result(input):
    cmds = [(*x.split(" "),) for x in input]
    a = 0
    x = 0
    y = 0
    
    for dir, val in cmds:
        if dir == "forward":
            x += int(val)
            y += int(val) * a
        elif dir == "down":
            a += int(val)
        elif dir == "up":
            a -= int(val)

    return x * y

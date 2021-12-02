# Advent of Code - Day 2 - Part One

def result(input):
    cmds = [(*x.split(" "),) for x in input]
    x = 0
    y = 0
    
    for dir, val in cmds:
        if dir == "forward":
            x += int(val)
        elif dir == "down":
            y += int(val)
        elif dir == "up":
            y -= int(val)

    return x * y

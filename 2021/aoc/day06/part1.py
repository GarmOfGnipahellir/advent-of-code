# Advent of Code - Day 6 - Part One

def result(input):
    fish = [int(n) for n in input[0].split(",")]

    numiter = 80
    values = []
    tmp = [0]
    for i in range(numiter):
        for j in range(len(tmp)):
            if tmp[j] == 0:
                tmp[j] = 6
                tmp.append(8)
            else:
                tmp[j] -= 1
        # print(i, len(tmp), tmp)
        if i > numiter - 7:
            values.append(len(tmp))
    
    res = 0
    for age in fish:
        res += values[len(values)-age-1]
    
    return res

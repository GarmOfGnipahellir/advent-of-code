# Advent of Code - Day 1 - Part One

def result(input):
    input = [int(x) for x in input]
    n = 0
    for i in range(1, len(input)):
        if input[i] > input[i - 1]:
            n += 1
    return n

# Advent of Code - Day 1 - Part Two

def result(input):
    input = [int(x) for x in input]
    n = 0

    sums = []
    for i in range(len(input) - 2):
        sums.append(input[i] + input[i+1] + input[i+2])

    for i in range(1, len(sums)):
        if sums[i] > sums[i - 1]:
            n += 1

    return n

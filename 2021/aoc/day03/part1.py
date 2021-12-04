# Advent of Code - Day 3 - Part One

def result(input):
    columns = []
    for i, ln in enumerate(input):
        for j, ch in enumerate(ln):
            if j == len(columns):
                columns.append([])
            if i == len(columns[j]):
                columns[j].append(int(ch))

    sums = [(sum(col), len(col)) for col in columns]

    gamma = ""
    epsilon = ""

    for x, n in sums:
        if x > n / 2:
            gamma += "1"
            epsilon += "0"
        else:
            gamma += "0"
            epsilon += "1"

    gamma = int(gamma, 2)
    epsilon = int(epsilon, 2)

    return gamma * epsilon

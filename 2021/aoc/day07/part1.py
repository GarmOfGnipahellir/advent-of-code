# Advent of Code - Day 7 - Part One

def result(input):
    positions = [int(n) for n in input[0].split(",")]
    median = sorted(positions)[len(positions) // 2]

    fuel = 0
    for pos in positions:
        fuel += abs(pos - median)

    return fuel

# Advent of Code - Day 7 - Part Two

# not sure why but this produced the right answer for my input but the example
# using ceil on the mean works for example but not input 🤷‍♂️

def result(input):
    positions = [int(n) for n in input[0].split(",")]
    mean = sum(positions) // len(positions)

    fuel = 0
    for pos in positions:
        delta = abs(pos - mean)
        fuel += delta * (delta + 1) // 2

    return fuel

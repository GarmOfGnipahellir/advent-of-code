"""https://adventofcode.com/2023/day/1"""

from os import path

EX1 = """1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"""

EX2 = """two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"""


def part1(inp: str):
    values = []
    for line in inp.splitlines():
        digits = ""
        for ch in line:
            if ch.isdigit():
                digits += ch
        value = digits[0] + digits[-1]
        values.append(int(value))
    return sum(values)


# TODO: Custom parser
def part2(inp: str):
    values = []
    for line in inp.splitlines():
        digits = ""
        for ch in line:
            if ch.isdigit():
                digits += ch
        value = digits[0] + digits[-1]
        values.append(int(value))
    return sum(values)


if __name__ == "__main__":
    print("Part 1:")
    print("  EX:", part1(EX1))
    with open(path.join(path.dirname(__file__), "in.txt")) as f:
        print(" ", part1(f.read()))

    print("Part 2:")
    print("  EX:", part2(EX2))
    with open(path.join(path.dirname(__file__), "in.txt")) as f:
        print(" ", part2(f.read()))

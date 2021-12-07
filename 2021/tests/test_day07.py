from aoc.day07 import part1, part2
from aoc.day07.__main__ import read_file

#
# --- Part One ---
#

def test_part1():
    assert part1.result(read_file("./resources/example.txt")) == 37

#
# --- Part Two ---
#

def test_part2():
    assert part2.result(read_file("./resources/example.txt")) == 168

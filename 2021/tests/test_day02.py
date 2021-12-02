from aoc.day02 import part1, part2
from aoc.day02.__main__ import read_file

#
# --- Part One ---
#

def test_part1():
    assert part1.result(read_file("./resources/example.txt")) == 150

#
# --- Part Two ---
#

def test_part2():
    assert part2.result(read_file("./resources/example.txt")) == 900

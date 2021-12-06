from aoc.day06 import part1, part2
from aoc.day06.__main__ import read_file

#
# --- Part One ---
#

def test_part1():
    assert part1.result(read_file("./resources/example.txt")) == 5934

#
# --- Part Two ---
#

def test_part2():
    assert part2.result(read_file("./resources/example.txt")) == 26984457539

from aoc.day17 import part1, part2
from aoc.day17.__main__ import read_file
from aoc.day17.part1 import Vec

#
# --- Part One ---
#

def test_vec():
    assert Vec(1, 2) + Vec(3, 4) == Vec(4, 6)

def test_part1():
    assert part1.result(read_file("./resources/example.txt")) == None

#
# --- Part Two ---
#

def test_part2():
    assert part2.result(read_file("./resources/example.txt")) == None

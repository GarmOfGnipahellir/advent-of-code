from aoc.$module_name import part1, part2
from aoc.$module_name.__main__ import read_file

#
# --- Part One ---
#

def test_part1():
    assert part1.result(read_file("./resources/example.txt")) == None

#
# --- Part Two ---
#

def test_part2():
    assert part2.result(read_file("./resources/example.txt")) == None

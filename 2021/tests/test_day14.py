from aoc.day14 import part1, part2
from aoc.day14.__main__ import read_file

#
# --- Part One ---
#

def test_polymerizer():
    polymerizer = part1.Polymerizer(read_file("./resources/example.txt"))
    assert polymerizer.polymer == "NNCB"
    polymerizer.step()
    assert polymerizer.polymer == "NCNBCHB"
    polymerizer.step()
    assert polymerizer.polymer == "NBCCNBBBCBHCB"
    polymerizer.step()
    assert polymerizer.polymer == "NBBBCNCCNBBNBNBBCHBHHBCHB"
    polymerizer.step()
    assert polymerizer.polymer == "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"

def test_part1():
    assert part1.result(read_file("./resources/example.txt")) == 1588

#
# --- Part Two ---
#

def test_part2():
    assert part2.result(read_file("./resources/example.txt")) == None

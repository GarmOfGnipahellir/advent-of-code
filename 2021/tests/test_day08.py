from aoc.day08 import part1, part2
from aoc.day08.__main__ import read_file
from aoc.day08.part2 import Digit

#
# --- Part One ---
#


def test_part1():
    assert part1.result(read_file("./resources/example.txt")) == 26

#
# --- Part Two ---
#


# def test_signal_map():
#     signalmap = part2.SignalMap(
#         ["acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab"])
#     assert "".join([val[0] for val in signalmap.dict.values()]) == "deafgbc"


def test_digit():
    assert Digit("dab") - Digit("ab") == Digit("d")
    assert len(Digit("abc")) == 3
    assert Digit("abc") == Digit("cab")


def test_part2():
    assert part2.result(read_file("./resources/example.txt")) == 61229

from aoc.day10 import part1, part2
from aoc.day10.__main__ import read_file
from aoc.day10.part1 import BracketError, Parser

#
# --- Part One ---
#


def test_parser():
    assert Parser("(]").parse().found == "]"
    assert Parser("{()()()>").parse().found == ">"
    assert Parser("(((()))}").parse().found == "}"
    assert Parser("<([]){()}[{}])").parse().found == ")"


def test_part1():
    assert part1.result(read_file("./resources/example.txt")) == 26397

#
# --- Part Two ---
#


def test_part2():
    assert part2.result(read_file("./resources/example.txt")) == 288957

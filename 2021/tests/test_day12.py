from aoc.day12 import part1, part2
from aoc.day12.__main__ import read_file

#
# --- Part One ---
#


def test_part1():
    assert part1.result([
        "start-A",
        "start-b",
        "A-c",
        "A-b",
        "b-d",
        "A-end",
        "b-end",
    ]) == 10
    assert part1.result([
        "dc-end",
        "HN-start",
        "start-kj",
        "dc-start",
        "dc-HN",
        "LN-dc",
        "HN-end",
        "kj-sa",
        "kj-HN",
        "kj-dc",
    ]) == 19
    assert part1.result(read_file("./resources/example.txt")) == 226

#
# --- Part Two ---
#


def test_part2():
    assert part2.result(read_file("./resources/example.txt")) == None

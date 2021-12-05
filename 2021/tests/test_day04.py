from aoc.day04 import part1, part2
from aoc.day04.__main__ import read_file

#
# --- Part One ---
#


def test_board():
    board = part1.Board([
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ])
    board.mark(3)
    board.mark(6)
    assert board.mark(9) == 243

    board = part1.Board([
        [3, 15,  0,  2, 22],
        [9, 18, 13, 17,  5],
        [19,  8,  7, 25, 23],
        [20, 11, 10, 24,  4],
        [14, 21, 16, 12,  6],
    ])
    assert board.find(16) == (2, 4)
    assert board.get(2, 4).num == 16


def test_part1():
    assert part1.result(read_file("./resources/example.txt")) == 4512

#
# --- Part Two ---
#


def test_part2():
    assert part2.result(read_file("./resources/example.txt")) == 1924

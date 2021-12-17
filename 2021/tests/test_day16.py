from aoc.day16 import part1, part2
from aoc.day16.__main__ import read_file

#
# --- Part One ---
#


def test_hex_to_bin():
    assert part1.hex_to_bin("D2FE28") == "110100101111111000101000"
    assert part1.hex_to_bin(
        "38006F45291200") == "00111000000000000110111101000101001010010001001000000000"


def test_parse():
    assert part1.parse(part1.hex_to_bin("D2FE28")) == part1.Literal(6, 2021)
    assert part1.parse(part1.hex_to_bin("38006F45291200")) == part1.Operator(1)


def test_part1():
    assert part1.result(["D2FE28"]) == 6
    assert part1.result(["38006F45291200"]) == 9
    assert part1.result(["8A004A801A8002F478"]) == 16
    assert part1.result(["620080001611562C8802118E34"]) == 12
    assert part1.result(["C0015000016115A2E0802F182340"]) == 23
    assert part1.result(["A0016C880162017C3686B18A3D4780"]) == 31

#
# --- Part Two ---
#


def test_part2():
    assert part2.result(read_file("./resources/example.txt")) == None

from aoc.day16 import part1, part2

#
# --- Part One ---
#


def test_hex_to_bin():
    assert part1.hex_to_bin("D2FE28") == "110100101111111000101000"
    assert part1.hex_to_bin(
        "38006F45291200") == "00111000000000000110111101000101001010010001001000000000"


def test_part1():
    assert part1.result(["D2FE28"]) == 6
    assert part1.result(["38006F45291200"]) == 9
    assert part1.result(["EE00D40C823060"]) == 14
    assert part1.result(["8A004A801A8002F478"]) == 16
    assert part1.result(["620080001611562C8802118E34"]) == 12
    assert part1.result(["C0015000016115A2E0802F182340"]) == 23
    assert part1.result(["A0016C880162017C3686B18A3D4780"]) == 31

#
# --- Part Two ---
#


def test_part2():
    assert part2.result(["C200B40A82"]) == 3
    assert part2.result(["04005AC33890"]) == 54
    assert part2.result(["880086C3E88112"]) == 7
    assert part2.result(["CE00C43D881120"]) == 9
    assert part2.result(["D8005AC2A8F0"]) == 1
    assert part2.result(["F600BC2D8F"]) == 0
    assert part2.result(["9C005AC2F8F0"]) == 0
    assert part2.result(["9C0141080250320F1802104A08"]) == 1

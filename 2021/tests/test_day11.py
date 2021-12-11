from aoc.day11 import part1, part2
from aoc.day11.__main__ import read_file

#
# --- Part One ---
#


def test_octopus():
    assert part1.Octopus(1, False) == part1.Octopus(1, False)


def test_octopus_map():
    octomap = part1.OctopusMap([
        "11111",
        "19991",
        "19191",
        "19991",
        "11111",
    ])
    print("initial")
    print(octomap)
    print()
    assert octomap == part1.OctopusMap([
        "11111",
        "19991",
        "19191",
        "19991",
        "11111",
    ])

    print("step 1")
    octomap.step()
    assert octomap == part1.OctopusMap([
        "34543",
        "40004",
        "50005",
        "40004",
        "34543",
    ])

    print("step 2")
    octomap.step()
    assert octomap == part1.OctopusMap([
        "45654",
        "51115",
        "61116",
        "51115",
        "45654",
    ])

def test_part1():
    assert part1.result(read_file("./resources/example.txt")) == 1656

#
# --- Part Two ---
#

def test_part2():
    assert part2.result(read_file("./resources/example.txt")) == 195

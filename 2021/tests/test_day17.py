from aoc.day17 import part1, part2
from aoc.day17.__main__ import read_file
from aoc.day17.part1 import Probe, Vec, Trajectory

#
# --- Part One ---
#


def test_vec():
    assert Vec(1, 2) + Vec(3, 4) == Vec(4, 6)


def test_trajectory():
    probe = Probe(Vec(0, 0), Vec(7, 2))
    trajectory = Trajectory(Vec(7, 2))
    assert trajectory.time_at_y(-7) == 7
    for i in range(100):
        probe.step()
        assert trajectory.evaluate(i+1) == probe.pos
    
    probe = Probe(Vec(0, 0), Vec(6, 3))
    trajectory = Trajectory(Vec(6, 3))
    assert trajectory.time_at_y(-9) == 9
    for i in range(100):
        probe.step()
        assert trajectory.evaluate(i+1) == probe.pos
    
    probe = Probe(Vec(0, 0), Vec(9, 0))
    trajectory = Trajectory(Vec(9, 0))
    assert trajectory.time_at_y(-10) == 5
    for i in range(100):
        probe.step()
        assert trajectory.evaluate(i+1) == probe.pos


def test_part1():
    assert part1.result(read_file("./resources/example.txt")) == 45

#
# --- Part Two ---
#


def test_part2():
    assert part2.result(read_file("./resources/example.txt")) == None

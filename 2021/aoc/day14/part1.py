# Advent of Code - Day 14 - Part One

from typing import Dict, List


class Polymerizer:
    template: str
    rules: Dict[str, str]
    polymer: str

    def __init__(self, input: List[str]) -> None:
        self.template = input[0]
        self.rules = {}
        for ln in input[2:]:
            src, el = ln.split(" -> ")
            self.rules[src] = el
        self.polymer = self.template

    def step(self) -> None:
        buf = list(self.polymer)
        cur = 1
        while cur < len(buf):
            pair = buf[cur-1] + buf[cur]
            buf.insert(cur, self.rules[pair])
            cur += 2
        self.polymer = "".join(buf)


def result(input):
    polymerizer = Polymerizer(input)
    for _ in range(10):
        polymerizer.step()

    elements = {}
    for el in polymerizer.polymer:
        if el not in elements.keys():
            elements[el] = 0
        elements[el] += 1

    most = max(elements.values())
    least = min(elements.values())
    return most - least

# Advent of Code - Day 10 - Part One

from typing import Union


PAIRS = {
    "{": "}",
    "[": "]",
    "(": ")",
    "<": ">",
}

SCORES = {
    ")": 3,
    "]": 57,
    "}": 1197,
    ">": 25137,
}


class BracketError(Exception):
    found: str
    expected: str
    position: int

    def __init__(self, found, expected, position) -> None:
        super().__init__(found, expected, position)
        self.found = found
        self.expected = expected
        self.position = position

    def __str__(self) -> str:
        return f"expected {self.expected} found {self.found} at {self.position}"


class Parser:
    line: str
    cursor: int

    def __init__(self, line) -> None:
        self.line = line
        self.cursor = 0

    def read(self) -> str:
        res = self.line[self.cursor]
        self.cursor += 1
        return res

    def parse(self) -> Union[None, BracketError]:
        try:
            ch = self.read()
            self.find(PAIRS[ch])
            return None
        except IndexError:
            return None
        except BracketError as e:
            return e

    def find(self, target: str) -> None:
        ch = self.read()
        if ch == target:
            return

        while ch in PAIRS.keys():
            self.find(PAIRS[ch])

            ch = self.read()
            if ch == target:
                return

        raise BracketError(ch, target, self.cursor)


def result(input):
    errors = []
    for ln in input:
        res = Parser(ln).parse()
        if res != None:
            errors.append(res)
    return sum([SCORES[e.found] for e in errors])

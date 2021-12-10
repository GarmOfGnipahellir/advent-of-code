# Advent of Code - Day 10 - Part Two

from typing import Union


PAIRS = {
    "{": "}",
    "[": "]",
    "(": ")",
    "<": ">",
}

SCORES = {
    ")": 1,
    "]": 2,
    "}": 3,
    ">": 4,
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
    written: str

    def __init__(self, line) -> None:
        self.line = line
        self.written = ""
        self.cursor = 0

    def read(self) -> str:
        res = self.line[self.cursor]
        self.cursor += 1
        return res

    def write(self, ch: str) -> None:
        self.line += ch
        self.written += ch
        self.cursor += 1

    def parse(self) -> Union[None, BracketError]:
        try:
            ch = self.read()
            while ch in PAIRS.keys():
                self.find(PAIRS[ch])

                if self.cursor == len(self.line):
                    break
                ch = self.read()

            return None
        except BracketError as e:
            return e

    def find(self, target: str) -> None:
        if self.cursor == len(self.line):
            self.write(target)
            return

        ch = self.read()
        if ch == target:
            return

        while ch in PAIRS.keys():
            self.find(PAIRS[ch])

            if self.cursor == len(self.line):
                self.write(target)
                return

            ch = self.read()
            if ch == target:
                return

        raise BracketError(ch, target, self.cursor)


def result(input):
    scores = []
    for ln in input:
        parser = Parser(ln)
        if parser.parse() != None:
            continue

        score = 0
        for ch in parser.written:
            score *= 5
            score += SCORES[ch]
        scores.append(score)

    return sorted(scores)[len(scores) // 2]

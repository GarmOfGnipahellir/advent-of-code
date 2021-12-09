# Advent of Code - Day 8 - Part Two

from typing_extensions import Self


class Digit:
    chars: str

    def __init__(self, chars: str) -> None:
        self.chars = chars

    def __str__(self) -> str:
        return self.chars

    def __eq__(self, other: Self) -> bool:
        if type(self) != type(other):
            return False
        return self.chars == other.chars

    def __sub__(self, other: Self) -> Self:
        tmp = self.chars
        for ch in other.chars:
            tmp = tmp.replace(ch, "")
        return Digit(tmp)


def result(input):
    input = [(*[spl.split() for spl in ln.split("|")],) for ln in input]
    return None

# Advent of Code - Day 8 - Part Two

from typing import List

DIGITS = [
    "abcefg",
    "cf",
    "acdeg",
    "acdfg",
    "bcdf",
    "abdfg",
    "abdefg",
    "acf",
    "abcdefg",
    "abcdfg",
]

LENGTHS = {
    2: [1],
    3: [7],
    4: [4],
    5: [2, 3, 5],
    6: [0, 6, 9],
    7: [8],
}


class SignalMap:
    dict = {
        "a": list("abcdefg"),
        "b": list("abcdefg"),
        "c": list("abcdefg"),
        "d": list("abcdefg"),
        "e": list("abcdefg"),
        "f": list("abcdefg"),
        "g": list("abcdefg"),
    }

    def __init__(self, inputs: List[str]):
        self.solve_map(inputs)

    def __getitem__(self, key: str) -> List[str]:
        return self.dict[key]

    def __setitem__(self, key: str, val: str) -> None:
        self.dict[key] = val

    def solve_digit(self, source: str, digit: str) -> None:
        for k in self.dict.keys():
            if k in digit:
                self[k] = list(filter(lambda ch: ch in source, self[k]))
            else:
                self[k] = list(filter(lambda ch: ch not in source, self[k]))

    def solve_map(self, inputs: List[str]) -> None:
        for inp in inputs:
            l = len(inp)
            for i in LENGTHS[l]:
                if l == 2 or l == 3 or l == 4 or l == 7:
                    self.solve_digit(inp, DIGITS[i])

        for k, v in self.dict.items():
            print(k, v)
        pass

    def map(self, input: str) -> str:
        return "".join([self[ch] for ch in input])


def print_digit(input):
    print("  {0}{0}{0}{0}  ".format("█" if "a" in input else " "))
    print("{0}{0}    {1}{1}".format("█" if "b" in input else " ",
          "█" if "c" in input else " "))
    print("{0}{0}    {1}{1}".format("█" if "b" in input else " ",
          "█" if "c" in input else " "))
    print("  {0}{0}{0}{0}  ".format("█" if "d" in input else " "))
    print("{0}{0}    {1}{1}".format("█" if "e" in input else " ",
          "█" if "f" in input else " "))
    print("{0}{0}    {1}{1}".format("█" if "e" in input else " ",
          "█" if "f" in input else " "))
    print("  {0}{0}{0}{0}  ".format("█" if "g" in input else " "))


def result(input):
    input = [(*[spl.split() for spl in ln.split("|")],) for ln in input]

    for entry in input:
        signalmap = SignalMap(entry[0])
        for inp in entry[0]:
            mapped = signalmap.map(inp)
            print(inp)
            print(mapped)
            print_digit(mapped)
        break

    return None

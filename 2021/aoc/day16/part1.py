# Advent of Code - Day 16 - Part One

from dataclasses import dataclass


@dataclass
class Packet:
    data: str
    version: int
    typeid: int


@dataclass
class Literal:
    version: int
    value: int


@dataclass
class Operator:
    version: int


def hex_to_bin(input: str) -> str:
    return f"{int(input, 16):b}".zfill(len(input) * 4)


def parse_literal(input: str) -> Literal:
    version = int(input[:3], 2)
    b = ""
    i = 0
    while True:
        grp = input[6+5*i:11+5*i]
        b += grp[1:]
        if grp[:1] == "0":
            break
        i += 1
    return Literal(version, int(b, 2))


def parse_operator(input: str) -> Operator:
    version = int(input[:3], 2)
    m = input[6:7]
    print(m)
    if m == "0":
        l = int(input[7:22], 2)
        print(l)
    else:
        pass
    return Operator(version)


def parse(input: str) -> Literal:
    typeid = int(input[3:6], 2)

    if typeid == 4:
        return parse_literal(input)
    else:
        return parse_operator(input)


def result(input):
    b = hex_to_bin(input[0])
    p = parse(b)
    return p.version

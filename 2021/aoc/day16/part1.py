# Advent of Code - Day 16 - Part One

from dataclasses import dataclass
from typing import List


@dataclass
class Packet:
    version: int
    typeid: int


@dataclass
class Literal(Packet):
    value: int


@dataclass
class Operator(Packet):
    children: List[Packet]


class Parser:
    cursor: int
    binary: str
    root: Packet

    def __init__(self, hex: str) -> None:
        self.cursor = 0
        self.binary = hex_to_bin(hex)
        self.root = None

    def read_bits(self, n: int) -> str:
        res = self.binary[self.cursor:self.cursor+n]
        self.cursor += n
        return res

    def parse_packet(self, parent: Operator = None) -> None:
        version = int(self.read_bits(3), 2)
        typeid = int(self.read_bits(3), 2)
        packet = Packet(version, typeid)

        # parse literal
        if typeid == 4:
            b = ""
            while True:
                grp = self.read_bits(5)
                b += grp[1:]
                if grp[:1] == "0":
                    break
            value = int(b, 2)
            packet = Literal(version, typeid, value)

        # parse operator
        else:
            packet = Operator(version, typeid, [])
            m = self.read_bits(1)
            if m == "0":
                l = int(self.read_bits(15), 2)
                endpos = self.cursor + l
                while self.cursor < endpos:
                    self.parse_packet(packet)
            else:
                l = int(self.read_bits(11), 2)
                for _ in range(l):
                    self.parse_packet(packet)

        # if there is no parent its the root
        if parent == None:
            self.root = packet
        else:
            parent.children.append(packet)

    def parse(self) -> None:
        self.parse_packet()


def hex_to_bin(hex: str) -> str:
    return f"{int(hex, 16):b}".zfill(len(hex) * 4)


def sum_version(packet: Packet) -> int:
    if not isinstance(packet, Operator):
        return packet.version

    res = packet.version
    for child in packet.children:
        res += sum_version(child)
    return res


def result(input):
    parser = Parser(input[0])
    parser.parse()
    return sum_version(parser.root)

# Advent of Code - Day 16 - Part Two

from typing import Iterable
from .part1 import *


def mul(iterable: Iterable[int]) -> int:
    iterator = iter(iterable)
    res = next(iterator)
    while True:
        try:
            res *= next(iterator)
        except StopIteration:
            break
    return res


def resolve_packet(packet: Packet) -> int:
    if isinstance(packet, Literal):
        return packet.value
    elif isinstance(packet, Operator):
        if packet.typeid == 0:
            return sum([resolve_packet(child) for child in packet.children])
        elif packet.typeid == 1:
            return mul([resolve_packet(child) for child in packet.children])
        elif packet.typeid == 2:
            return min([resolve_packet(child) for child in packet.children])
        elif packet.typeid == 3:
            return max([resolve_packet(child) for child in packet.children])
        elif packet.typeid == 5:
            if resolve_packet(packet.children[0]) > resolve_packet(packet.children[1]):
                return 1
            else:
                return 0
        elif packet.typeid == 6:
            if resolve_packet(packet.children[0]) < resolve_packet(packet.children[1]):
                return 1
            else:
                return 0
        elif packet.typeid == 7:
            if resolve_packet(packet.children[0]) == resolve_packet(packet.children[1]):
                return 1
            else:
                return 0
        else:
            raise NotImplementedError(
                f"operators with type id {packet.typeid} are not supported")
    else:
        raise ValueError("bare packets has no value")


def result(input):
    parser = Parser(input[0])
    parser.parse()
    return resolve_packet(parser.root)

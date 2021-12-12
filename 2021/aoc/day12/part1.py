# Advent of Code - Day 12 - Part One

from dataclasses import dataclass
from typing import Dict, List


@dataclass
class Node:
    name: str
    links: List[str]


class Graph:
    nodes: Dict[str, Node]

    def __init__(self, input: List[str]) -> None:
        self.nodes = {}
        for ln in input:
            a, b = ln.split("-")
            na = self.nodes.get(a)
            if na == None:
                na = Node(a, [])
                self.nodes[a] = na
            na.links.append(b)
            nb = self.nodes.get(b)
            if nb == None:
                nb = Node(b, [])
                self.nodes[b] = nb
            nb.links.append(a)

    def remove(self, name: str) -> None:
        node = self.nodes.get(name)
        if node == None:
            return

        for other in [self.nodes[n] for n in node.links]:
            other.links.remove(name)
        del self.nodes[name]

    def find_path(self, path: List[str]) -> None:
        node = self.nodes[path[len(path) - 1]]
        for link in node.links:
            if link not in path:
                path.append(link)
                if link != "end":
                    self.find_path(path)
                return

    def find_paths(self, paths: List[List[str]]) -> None:
        pass


def result(input):
    graph = Graph(input)

    # cull irrelevant nodes
    remove = []
    for node in graph.nodes.values():
        if len(node.links) <= 1:
            remove.append(node.name)
    for name in remove:
        graph.remove(name)

    path = ["start"]
    graph.find_path(path)
    print(path)

    return None

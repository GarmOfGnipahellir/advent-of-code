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

    def find_paths_helper(self, current: str, path: List[str], paths: List[List[str]]) -> None:
        path.append(current)

        if current == "end":
            paths.append([*path])
        else:
            for next in self.nodes[current].links:
                if next.isupper() or next not in path:
                    self.find_paths_helper(next, path, paths)

        path.pop()

    def find_paths(self) -> List[List[str]]:
        path = []
        paths = []
        self.find_paths_helper("start", path, paths)
        return paths


def result(input):
    graph = Graph(input)
    paths = graph.find_paths()
    return len(paths)

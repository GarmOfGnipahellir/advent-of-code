# Advent of Code - Day 4 - Part One

from typing import List, NamedTuple, Tuple, Union
from colorama import Fore, Style


class Cell(NamedTuple):
    num: int
    mark: bool


class Board:
    def __init__(self, rows: List[List[int]]):
        self.size = len(rows)
        self.rows = [list(map(lambda x: Cell(x, False), row)) for row in rows]

    def __str__(self) -> str:
        return "\n".join([" ".join([f"{Fore.GREEN if cell.mark else Fore.RED}{cell.num}{Style.RESET_ALL}".rjust(2) for cell in row]) for row in self.rows])

    def get(self, x: int, y: int) -> Union[Cell, None]:
        if y < 0 or y >= self.size:
            return None
        if x < 0 or x >= self.size:
            return None
        return self.rows[y][x]

    def set(self, x: int, y: int, cell: Cell):
        if y < 0 or y >= self.size:
            return
        if x < 0 or x >= self.size:
            return
        self.rows[y][x] = cell

    def get_row(self, y: int) -> Union[List[Cell], None]:
        if y < 0 or y >= self.size:
            return None
        return self.rows[y]

    def get_col(self, x: int) -> Union[List[Cell], None]:
        if x < 0 or x >= self.size:
            return None
        return [row[x] for row in self.rows]

    def find(self, n: int) -> Union[Tuple[int, int], None]:
        for x in range(self.size):
            for y in range(self.size):
                if self.get(x, y).num == n:
                    return (x, y)
        return None

    def mark(self, n: int) -> Union[int, None]:
        found = self.find(n)
        if found == None:
            return None

        x, y = found
        cell = self.get(x, y)
        if cell != None:
            self.set(x, y, Cell(cell.num, True))

            if sum([1 if cell.mark else 0 for cell in self.get_col(x)]) == self.size:
                return self.get_score(n)

            if sum([1 if cell.mark else 0 for cell in self.get_row(y)]) == self.size:
                return self.get_score(n)

        return None

    def get_score(self, n: int) -> int:
        cells = sum(self.rows, [])
        cells = list(filter(lambda cell: not cell.mark, cells))
        cells = [cell.num for cell in cells]
        return sum(cells) * n


def result(input):
    rns = [int(n) for n in input[0].split(",")]

    nboards = (len(input) - 2) // 6
    boards: List[Board] = []
    for i in range(nboards):
        startindex = i * 6 + 2
        endindex = startindex + 5
        lines = input[startindex:endindex]
        lines = [[int(n) for n in ln.split()] for ln in lines]
        board = Board(lines)
        boards.append(board)

    for n in rns:
        for board in boards:
            score = board.mark(n)
            if score != None:
                return score

    return None

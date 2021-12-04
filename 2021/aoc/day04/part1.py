# Advent of Code - Day 4 - Part One

from typing import List, Tuple, Union


class Board:
    def __init__(self, rows: List[List[int]]):
        self.size = len(rows)
        self.rows = rows

    def __str__(self) -> str:
        return "\n".join([" ".join([str(n).rjust(2) for n in row]) for row in self.rows])

    def get(self, x: int, y: int) -> Union[int, None]:
        if y < 0 or y >= self.size:
            return None
        if x < 0 or x >= self.size:
            return None
        return self.rows[y][x]

    def get_row(self, y: int) -> Union[List[int], None]:
        if y < 0 or y >= self.size:
            return None
        return self.rows[y]

    def get_col(self, x: int) -> Union[List[int], None]:
        if x < 0 or x >= self.size:
            return None
        return [row[x] for row in self.rows]

    def find(self, n: int) -> Union[Tuple[int, int], None]:
        for x in range(self.size):
            for y in range(self.size):
                if self.get(x, y) == n:
                    return (x, y)
        return None

    def check_win(self, nums: List[int]) -> bool:
        lastnum = nums[len(nums) - 1]
        found = self.find(lastnum)
        if found == None:
            return False
        x, y = found

        res = True
        for n in self.get_col(x):
            if n not in nums:
                res = False
                break

        res = True
        for n in self.get_row(y):
            if n not in nums:
                res = False
                break

        return res

    def calc_score(self, nums: List[int]) -> int:
        sum = 0
        for x in range(self.size):
            for y in range(self.size):
                n = self.get(x, y)
                if n not in nums:
                    sum += n
        return sum * nums[len(nums) - 1]


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

    nums: List[int] = []
    for n in rns:
        nums.append(n)

        for board in boards:
            if board.check_win(nums):
                return board.calc_score(nums)

    return None

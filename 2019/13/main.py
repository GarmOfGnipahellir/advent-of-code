import curses
from curses import wrapper
import re
from os import path
__dir__ = path.dirname(__file__)


def sign(num):
    if num > 0:
        return 1
    elif num < 0:
        return -1
    else:
        return 0


def OpAdd(computer, param_modes):
    param_modes = computer.resolve_param_modes(param_modes, 3)

    param1 = computer.get_param_value(1, param_modes)
    param2 = computer.get_param_value(2, param_modes)
    param3 = computer.get_param_outpos(3, param_modes)

    computer.set_value(param3, param1 + param2)

    computer.pos += 4


def OpMultiply(computer, param_modes):
    param_modes = computer.resolve_param_modes(param_modes, 3)

    param1 = computer.get_param_value(1, param_modes)
    param2 = computer.get_param_value(2, param_modes)
    param3 = computer.get_param_outpos(3, param_modes)

    computer.set_value(param3, param1 * param2)

    computer.pos += 4


def OpInput(computer, param_modes):
    inval = computer.args[computer.arg]
    computer.arg += 1

    param_modes = computer.resolve_param_modes(param_modes, 1)

    param1 = computer.get_param_outpos(1, param_modes)

    computer.set_value(param1, inval)

    computer.pos += 2


def OpOutput(computer, param_modes):
    param_modes = computer.resolve_param_modes(param_modes, 1)

    param1 = computer.get_param_value(1, param_modes)

    computer.pos += 2

    return param1


def OpJumpIfTrue(computer, param_modes):
    param_modes = computer.resolve_param_modes(param_modes, 2)

    param1 = computer.get_param_value(1, param_modes)
    param2 = computer.get_param_value(2, param_modes)

    if param1 != 0:
        computer.pos = param2
    else:
        computer.pos += 3


def OpJumpIfFalse(computer, param_modes):
    param_modes = computer.resolve_param_modes(param_modes, 2)

    param1 = computer.get_param_value(1, param_modes)
    param2 = computer.get_param_value(2, param_modes)

    if param1 == 0:
        computer.pos = param2
    else:
        computer.pos += 3


def OpLessThan(computer, param_modes):
    param_modes = computer.resolve_param_modes(param_modes, 3)

    param1 = computer.get_param_value(1, param_modes)
    param2 = computer.get_param_value(2, param_modes)
    param3 = computer.get_param_outpos(3, param_modes)

    if param1 < param2:
        computer.set_value(param3, 1)
    else:
        computer.set_value(param3, 0)

    computer.pos += 4


def OpEquals(computer, param_modes):
    param_modes = computer.resolve_param_modes(param_modes, 3)

    param1 = computer.get_param_value(1, param_modes)
    param2 = computer.get_param_value(2, param_modes)
    param3 = computer.get_param_outpos(3, param_modes)

    if param1 == param2:
        computer.set_value(param3, 1)
    else:
        computer.set_value(param3, 0)

    computer.pos += 4


def OpSetRel(computer, param_modes):
    param_modes = computer.resolve_param_modes(param_modes, 1)

    param1 = computer.get_param_value(1, param_modes)

    computer.rel += param1

    computer.pos += 2


class IntcodeComputer:
    def __init__(self, intcode):
        self.intcode = intcode
        self.halted = False
        self.pos = 0
        self.rel = 0
        self.arg = 0
        self.args = []
        self.ops = {
            1: OpAdd,
            2: OpMultiply,
            3: OpInput,
            4: OpOutput,
            5: OpJumpIfTrue,
            6: OpJumpIfFalse,
            7: OpLessThan,
            8: OpEquals,
            9: OpSetRel,
        }

    @staticmethod
    def resolve_param_modes(param_modes, num_params):
        param_modes.reverse()
        for i in range(num_params-len(param_modes)):
            param_modes.append(0)
        return param_modes

    def get_param_value(self, num, param_modes):
        value = self.intcode[self.pos + num]
        if param_modes[num-1] == 0:
            self.alloc(value)
            value = self.intcode[value]
        elif param_modes[num-1] == 2:
            value = self.intcode[self.rel + value]
        return value

    def get_param_outpos(self, num, param_modes):
        outpos = self.intcode[self.pos + num]
        if param_modes[num-1] == 2:
            outpos = self.rel + outpos
        return outpos

    def parse_instruction(self, instruction):
        instruction = str(instruction)

        if len(instruction) <= 2:
            opcode = int(instruction)
            param_modes = []
        else:
            matches = re.match(r'(\d+)(\d{2})$', instruction)
            opcode = int(matches.group(2))
            param_modes = [int(x) for x in list(matches.group(1))]

        return (opcode, param_modes)

    def set_value(self, pos, value):
        self.alloc(pos)
        self.intcode[pos] = value

    def alloc(self, pos):
        last = len(self.intcode) - 1
        if pos > last:
            self.intcode.extend([0 for _ in range(pos - last)])

    def execute(self, *args):
        self.arg = 0
        self.args = args
        while True:
            instruction = self.intcode[self.pos]
            if instruction == 99:
                self.halted = True
                return
            (opcode, param_modes) = self.parse_instruction(instruction)
            out = self.ops[opcode](self, param_modes)
            if out != None:
                return out


with open(f'{__dir__}/in', 'r') as f:
    intcode = [int(x) for x in f.read().strip().split(',')]

intcode[0] = 2
score = 0
tiles = {}


def main(stdscr):
    score = 0
    joyin = 0
    paddle = None
    ball = None
    comp = IntcodeComputer(intcode)
    while True:
        x = comp.execute(joyin)
        if x == None:
            break

        y = comp.execute()
        if y == None:
            break

        tile = comp.execute()
        if tile == None:
            break

        if x == -1 and y == 0:
            score = tile
        else:
            tiles[(x, y)] = tile

            if tile == 3:
                paddle = (x, y)
            elif tile == 4:
                ball = (x, y)
                if paddle != None:
                    joyin = sign(ball[0] - paddle[0])
        
        stdscr.clear()
        for pos, tile in tiles.items():
            char = ' '
            if tile == 1:
                char = '░'
            elif tile == 2:
                char = '▒'
            elif tile == 3:
                char = '▔'
            elif tile == 4:
                char = '█'

            stdscr.addstr(pos[1], pos[0], char)
            stdscr.addstr(0, 36, 'Score:')
            stdscr.addstr(1, 36, str(score))
        stdscr.refresh()
    
    while True:
        pass

curses.initscr()
curses.noecho()
wrapper(main)
curses.echo()
curses.endwin()

print(score)
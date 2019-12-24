import re
from math import floor, pow, log2
from os import path
__dir__ = path.dirname(__file__)


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


class Turtle:
    UP = [0, -1]
    RIGHT = [1, 0]
    DOWN = [0, 1]
    LEFT = [-1, 0]
    DIRS = [UP, RIGHT, DOWN, LEFT]

    def __init__(self):
        self.pos = [0, 0]
        self.direction = 0
        self.painted = {
            (0, 0): 1
        }
    
    def get_paint_at_pos(self, pos):
        if tuple(pos) in self.painted:
            return self.painted[tuple(pos)]
        else:
            return 0
    
    def get_paint(self):
        return self.get_paint_at_pos(self.pos)

    def paint(self, color):
        self.painted[tuple(self.pos)] = color

    def walk(self, direction):
        if direction == 0:
            self.direction = (self.direction - 1) % 4
        elif direction == 1:
            self.direction = (self.direction + 1) % 4

        self.pos[0] += self.DIRS[self.direction][0]
        self.pos[1] += self.DIRS[self.direction][1]

    def __str__(self):
        min_x = 2**128
        min_y = 2**128
        max_x = -2**128
        max_y = -2**128
        for pos, color in self.painted.items():
            if pos[0] < min_x:
                min_x = pos[0]
            if pos[0] > max_x:
                max_x = pos[0]
            if pos[1] < min_y:
                min_y = pos[1]
            if pos[1] > max_y:
                max_y = pos[1]
        
        num_rows = max_y - min_y + 1
        num_cols = max_x - min_x + 1

        rows = []
        for y in range(num_rows):
            row = ''
            for x in range(num_cols):
                pos = [x + min_x, y + min_y]
                color = self.get_paint_at_pos(pos)
                if color == 1:
                    row += '▓'
                else:
                    row += '░'
            rows.append(row)
        
        return '\n'.join(rows)



with open(f'{__dir__}/in', 'r') as f:
    intcode = [int(x) for x in f.read().strip().split(',')]

comp = IntcodeComputer(intcode)
robot = Turtle()
output = 0
while output != None:
    color = comp.execute(robot.get_paint())
    direction = comp.execute()

    robot.paint(color)
    robot.walk(direction)

    output = direction

print(robot)

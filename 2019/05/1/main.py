import re
from os import path
__dir__ = path.dirname(__file__)

with open(f'{__dir__}/in', 'r') as f:
    intcode = [int(x) for x in f.read().split(',')]


def OpAdd(computer, param_modes):
    param_modes = computer.resolve_param_modes(param_modes, 3)

    param1 = computer.intcode[computer.pos + 1]
    if param_modes[0] == 0:
        param1 = computer.intcode[param1]

    param2 = computer.intcode[computer.pos + 2]
    if param_modes[1] == 0:
        param2 = computer.intcode[param2]
    
    param3 = computer.intcode[computer.pos + 3]

    computer.intcode[param3] = param1 + param2

    computer.pos += 4


def OpMultiply(computer, param_modes):
    param_modes = computer.resolve_param_modes(param_modes, 3)

    param1 = computer.intcode[computer.pos + 1]
    if param_modes[0] == 0:
        param1 = computer.intcode[param1]

    param2 = computer.intcode[computer.pos + 2]
    if param_modes[1] == 0:
        param2 = computer.intcode[param2]
    
    param3 = computer.intcode[computer.pos + 3]

    computer.intcode[param3] = param1 * param2

    computer.pos += 4


def OpInput(computer, param_modes):
    inval = computer.args[computer.arg]
    computer.arg += 1

    outpos = computer.intcode[computer.pos + 1]
    computer.intcode[outpos] = inval

    computer.pos += 2


def OpOutput(computer, param_modes):
    param_modes = computer.resolve_param_modes(param_modes, 1)
    
    param1 = computer.intcode[computer.pos + 1]
    if param_modes[0] == 0:
        param1 = computer.intcode[param1]

    print(param1)

    computer.pos += 2


class IntcodeComputer:
    def __init__(self, intcode):
        self.intcode = intcode
        self.pos = 0
        self.arg = 0
        self.args = []
        self.ops = {
            1: OpAdd,
            2: OpMultiply,
            3: OpInput,
            4: OpOutput,
        }

    @staticmethod
    def resolve_param_modes(param_modes, num_params):
        param_modes.reverse()
        for i in range(num_params-len(param_modes)):
            param_modes.append(0)
        return param_modes

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

    def execute(self, *args):
        self.arg = 0
        self.args = args
        while True:
            instruction = self.intcode[self.pos]
            if instruction == 99:
                break
            (opcode, param_modes) = self.parse_instruction(instruction)
            self.ops[opcode](self, param_modes)


IntcodeComputer(intcode).execute(1)

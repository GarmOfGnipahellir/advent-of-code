import re
from math import floor, pow, log2
from os import path
__dir__ = path.dirname(__file__)


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

    computer.pos += 2

    return param1


def OpJumpIfTrue(computer, param_modes):
    param_modes = computer.resolve_param_modes(param_modes, 2)

    param1 = computer.intcode[computer.pos + 1]
    if param_modes[0] == 0:
        param1 = computer.intcode[param1]

    param2 = computer.intcode[computer.pos + 2]
    if param_modes[1] == 0:
        param2 = computer.intcode[param2]

    if param1 != 0:
        computer.pos = param2
    else:
        computer.pos += 3


def OpJumpIfFalse(computer, param_modes):
    param_modes = computer.resolve_param_modes(param_modes, 2)

    param1 = computer.intcode[computer.pos + 1]
    if param_modes[0] == 0:
        param1 = computer.intcode[param1]

    param2 = computer.intcode[computer.pos + 2]
    if param_modes[1] == 0:
        param2 = computer.intcode[param2]

    if param1 == 0:
        computer.pos = param2
    else:
        computer.pos += 3


def OpLessThan(computer, param_modes):
    param_modes = computer.resolve_param_modes(param_modes, 3)

    param1 = computer.intcode[computer.pos + 1]
    if param_modes[0] == 0:
        param1 = computer.intcode[param1]

    param2 = computer.intcode[computer.pos + 2]
    if param_modes[1] == 0:
        param2 = computer.intcode[param2]

    param3 = computer.intcode[computer.pos + 3]

    if param1 < param2:
        computer.intcode[param3] = 1
    else:
        computer.intcode[param3] = 0

    computer.pos += 4


def OpEquals(computer, param_modes):
    param_modes = computer.resolve_param_modes(param_modes, 3)

    param1 = computer.intcode[computer.pos + 1]
    if param_modes[0] == 0:
        param1 = computer.intcode[param1]

    param2 = computer.intcode[computer.pos + 2]
    if param_modes[1] == 0:
        param2 = computer.intcode[param2]

    param3 = computer.intcode[computer.pos + 3]

    if param1 == param2:
        computer.intcode[param3] = 1
    else:
        computer.intcode[param3] = 0

    computer.pos += 4


class IntcodeComputer:
    def __init__(self, intcode):
        self.intcode = intcode
        self.halted = False
        self.pos = 0
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
                self.halted = True
                return
            (opcode, param_modes) = self.parse_instruction(instruction)
            out = self.ops[opcode](self, param_modes)
            if out != None:
                return out


def swap(iterator, index1, index2):
    tmp = iterator[index1]
    iterator[index1] = iterator[index2]
    iterator[index2] = tmp


with open(f'{__dir__}/in', 'r') as f:
    intcode = [int(x) for x in f.read().strip().split(',')]

phase_settings = [5, 6, 7, 8, 9]
sequences = [phase_settings]
indexes = [0 for i in phase_settings]

i = 0
while i < len(phase_settings):
    if indexes[i] < i:
        swap(phase_settings, 0 if i % 2 == 0 else indexes[i], i)
        sequences.append(phase_settings.copy())
        indexes[i] += 1
        i = 0
    else:
        indexes[i] = 0
        i += 1

record = 0
for setting in sequences:
    print(setting)
    
    computers = [IntcodeComputer(intcode.copy()) for _ in range(5)]
    
    prev = 0
    for i in range(0, len(computers)):
        val = setting[i]
        temp = prev
        print()
        print(i, computers[i].intcode)
        prev = computers[i].execute(val, prev)
        print(f'{val}, {temp}', '->', i, '->', prev)
        print(i, computers[i].intcode)

    i = 0
    while True:
        allHalted = True
        for comp in computers:
            if not comp.halted:
                allHalted = False
        
        if allHalted:
            break

        if computers[i].halted:
            print(i, 'is halted')
            continue

        val = setting[i]
        temp = prev
        print()
        print(i, computers[i].intcode)
        output = computers[i].execute(prev)
        if output == None:
            i = (i + 1) % len(computers)
            continue
        prev = output
        print(f'{temp}', '->', i, '->', prev)
        print(i, computers[i].intcode)
        i = (i + 1) % len(computers)

    if prev > record:
        record = prev

print(record)

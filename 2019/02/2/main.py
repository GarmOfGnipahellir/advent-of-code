from os import path
__dir__ = path.dirname(__file__)

for noun in range(0, 99):
    for verb in range(0, 99):
        with open(f'{__dir__}/in', 'r') as f:
            intcode = [int(x) for x in f.read().split(',')]
        
        intcode[1] = noun
        intcode[2] = verb

        for i in range(0, len(intcode), 4):
            opcode = intcode[i]

            if opcode == 99:
                break

            input1 = intcode[intcode[i+1]]
            input2 = intcode[intcode[i+2]]
            output_pos = intcode[i+3]

            output = 0
            if opcode == 1:
                output = input1 + input2
            elif opcode == 2:
                output = input1 * input2
            else:
                raise RuntimeError(f'encountered opcode: {opcode}')

            intcode[output_pos] = output

        if intcode[0] == 19690720:
            print(f'{intcode[0]} = f({noun}, {verb})')
            print(100 * noun + verb)
            break

from os import path
__dir__ = path.dirname(__file__)


with open(f'{__dir__}/in', 'r') as f:
    lines = f.readlines()

    # read image size
    width = int(lines[0])
    height = int(lines[1])
    # pixels per level
    ppl = width * height

    # read layers
    image = [int(x) for x in list(lines[2])]
    layers = []
    for i in range(0, len(image), ppl):
        layer = image[i:i + ppl]
        layers.append(layer)
    
    # print layers for fun
    for num, layer in enumerate(layers):
        print('Layer', num)
        for i in range(0, len(layer), width):
            row = layer[i:i + width]
            print(' ', row)

# find layer with most zeroes
record = 2**128
record_layer = -1
for num, layer in enumerate(layers):
    count = 0
    for pixel in layer:
        if pixel == 0:
            count += 1
    
    if count < record:
        record = count
        record_layer = num

# make sure we found our layer
assert(record < 2**128)
assert(record_layer >= 0)

# find number of 1's and 2's
num1 = 0
num2 = 0
for pixel in layers[record_layer]:
    if pixel == 1:
        num1 += 1
    elif pixel == 2:
        num2 += 1

# print answer
print()
print(num1 * num2)
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
            print(' ', ''.join([str(x) for x in row])
                .replace('0', '░')
                .replace('1', '▓')
                .replace('2', ' '))

# render back to frant
layers.reverse()
render = layers[0]
for i in range(1, len(layers)):
    layer = layers[i]
    for j in range(len(render)):
        fp = layer[j]
        if fp != 2:
            render[j] = fp
        else:
            continue

print('Render')
for i in range(0, len(render), width):
    row = render[i:i + width]
    print(' ', ''.join([str(x) for x in row])
        .replace('0', '░')
        .replace('1', '▓')
        .replace('2', ' '))
# Advent of Code - Day 6 - Part One

def result(input):
    fish = [int(n) for n in input[0].split(",")]
    print(fish)

    numiter = 80

    for i in range(numiter//7):
        for j in range(len(fish)):
            fish.append(8 + fish[j] - 6)
        print((i + 1) * 7)
    iterleft = numiter - (numiter//7) * 7
    
    for j in range(len(fish)):
        fish[j] -= iterleft
        if fish[j] < 0:
            fish.append(9 + fish[j])
            fish[j] += 7
    
    return len(fish)

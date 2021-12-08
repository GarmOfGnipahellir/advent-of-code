# Advent of Code - Day 8 - Part One

def result(input):
    input = [(*[spl.split() for spl in ln.split("|")],) for ln in input]

    count = 0
    for entry in input:
        for output in entry[1]:
            loutput = len(output)
            if loutput == 2 or loutput == 4 or loutput == 3 or loutput == 7:
                count += 1

    return count

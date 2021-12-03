# Advent of Code - Day 3 - Part Two

def result(input):
    nbits = len(input[0])

    nums = input
    oxy = None
    for i in range(nbits):
        n1 = sum([int(ln[i]) for ln in nums])
        n0 = len(nums) - n1

        mask = 1 << (nbits - i - 1)
        invmask = (2 ** nbits - 1) ^ mask

        keep = []
        for ln in nums:
            n = int(ln, 2)
            if n1 >= n0:
                if n & mask == mask:
                    keep.append(ln)
            else:
                if n | invmask == invmask:
                    keep.append(ln)
        nums = keep

        if len(nums) == 1:
            oxy = int(nums[0], 2)
            break
    
    nums = input
    co2 = None
    for i in range(nbits):
        n1 = sum([int(ln[i]) for ln in nums])
        n0 = len(nums) - n1

        mask = 1 << (nbits - i - 1)
        invmask = (2 ** nbits - 1) ^ mask

        keep = []
        for ln in nums:
            n = int(ln, 2)
            if n0 > n1:
                if n & mask == mask:
                    keep.append(ln)
            else:
                if n | invmask == invmask:
                    keep.append(ln)
        nums = keep

        if len(nums) == 1:
            co2 = int(nums[0], 2)
            break

    return oxy * co2

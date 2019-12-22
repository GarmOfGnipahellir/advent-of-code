# six digits
# two adjacent same
# never decrease from left to right

input = [273025, 767253]

# first and last valid is
# 277777 - 699999


def fix_decreasing(num):
    result = list(str(num))
    for i, char in enumerate(result):
        if i > 0:
            cur = int(char)
            prev = int(result[i-1])
            if cur < prev:
                result[i] = str(prev)
    return int(''.join(result))


def has_double(num):
    num = [int(x) for x in list(str(num))]
    seqs = []
    count = 0
    for i in range(1, len(num)):
        prev = num[i-1]
        cur = num[i]
        count += 1
        if prev != cur:
            seqs.append(count)
            count = 0
        elif i == len(num) - 1:
            if prev == cur:
                count += 1
            seqs.append(count)
    return 2 in seqs


unique = []
for i in range(input[0], input[1]):
    fixed = fix_decreasing(i)
    if fixed not in unique and fixed < input[1]:
        unique.append(fixed)

valid = []
for num in unique:
    if has_double(num):
        valid.append(num)

print(len(valid))

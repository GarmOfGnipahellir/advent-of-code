inp = open("./in00", mode="r").readlines()
inp = [int(line.strip()) for line in inp]

print(inp)

preamble = 25

invalid = -1

for i, n in enumerate(inp[preamble:]):
    to_check = inp[i:i + preamble]
    nvalid = 0
    for j, m in enumerate(to_check):
        for k, o in enumerate(to_check):
            if i == j: continue

            if m + o == n:
                nvalid += 1
    
    if nvalid == 0:
        invalid = n
        break

for n in range(1, len(inp)):
    max_start = len(inp) - n - 1
    print(n, max_start)
    found = False
    for m in range(max_start + 1):
        print(" ", m, m+n)
        nums = inp[m:m+n+1]
        sm = sum(nums)
        if sm == invalid:
            print(nums)
            print(sm)
            print(max(nums) + min(nums))
            found = True
            break
    
    if found:
        break
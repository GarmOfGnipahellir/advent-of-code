from math import pow

inp = open("./ex00", mode="r").readlines()
inp = [int(line.strip()) for line in inp]

chain = inp.copy()
chain.append(0)
chain.append(max(chain) + 3)
chain.sort()

last = max(chain)

print(chain)

def is_valid(lst):
    for i in range(1, len(lst)):
        n1 = lst[i-1]
        n2 = lst[i]

        if n2 - n1 > 3:
            return False
    return True

def recursive_remove(lst, valid):
    for i in range(len(lst) - 2, 0, -1):
        pre = lst[i-1]
        nxt = lst[i+1]

        if nxt - pre <= 3:
            new_lst = lst.copy()
            new_lst.remove(new_lst[i])
            if new_lst not in valid:
                valid.append(new_lst)
                recursive_remove(new_lst, valid)

# valid = [chain]
# recursive_remove(chain, valid)
# print(len(valid))

removable = []
for i in range(1, len(chain)-1):
    pre = chain[i-1]
    nxt = chain[i+1]

    if nxt - pre <= 3:
        removable.append(i)
print(removable)

diffs = []
for i in removable:
    pre = chain[i-1]
    cur = chain[i]
    nxt = chain[i+1]
    
    diff = max(cur-pre, nxt-cur)

    diffs.append(diff)
print(diffs)

pairs = 0
for i in range(len(removable)-1):
    cur = chain[i]
    nxt = chain[i+1]
    if cur + 1 == nxt and (diffs[i] == 1 and diffs[i+1] == 1):
        pairs += 1

print(len(removable) + pairs)

# ans = 0
# from itertools import compress, product
# combinations = ( set(compress(removable, mask)) for mask in product(*[[0,1]]*len(removable)) )
# for comb in combinations:
#     new_lst = chain.copy()
#     for i in comb:
#         new_lst.remove(chain[i])
#     if is_valid(new_lst):
#         ans += 1
# print(ans)
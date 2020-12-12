inp = open("./ex01", mode="r").readlines()
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

def recursive_test(lst, valid):
    if len(lst) <= 3:
        return 0

    n = 0

    for i in range(1, len(lst) - 1):
        test = lst.copy()
        test.remove(test[i])

        if is_valid(test):
            if test not in valid:
                valid.append(test)
                n += 1
                print(test)
        
        n += recursive_test(test, valid)
    
    return n

valid = []
print(recursive_test(chain, valid) + 1)
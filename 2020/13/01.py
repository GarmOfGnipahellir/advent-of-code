from math import inf

inp = open("./in00", mode="r").readlines()
inp = [line.strip() for line in inp]

earliest_departure = int(inp[0])
busses = [int(x) for x in inp[1].split(",") if x != "x"]

best_bus = -1
best_time = inf
for bus in busses:
    t = 0
    while t < earliest_departure:
        t += bus
    
    if t < best_time:
        best_time = t
        best_bus = bus

print(best_bus, best_time)
print(best_time - earliest_departure)
print((best_time - earliest_departure) * best_bus)
from math import inf
from dataclasses import dataclass

@dataclass
class Bus:
    bid: int
    oid: int
    step: int = 0

    def get_time(self) -> int:
        return self.bid * self.step

    def step_to_earliest_time_past(self, t: int) -> int:
        while self.get_time() < t:
            self.step += 1
        return self.get_time()

    def get_time_equality(self) -> int:
        return self.get_time() - self.oid

inp = open("./in00", mode="r").readlines()
inp = [line.strip() for line in inp]

earliest_departure = int(inp[0])
busses = [Bus(int(x), i) for i, x in enumerate(inp[1].split(",")) if x != "x"]

for i, b in enumerate(busses):
    print(i, b)

n = 0
while True:
    n += 1
    if n % 100000 == 0:
        print(n)
        for i, b in enumerate(busses):
            print(i, b)
    time_equalized = True

    for bus1 in busses:
        for bus2 in busses:
            if bus1.get_time_equality() != bus2.get_time_equality():
                time_equalized = False
                break
        if not time_equalized:
            break
    
    for i in range(len(busses)-1, -1, -1):
        if i == 0:
            busses[i].step += 1
            break
        elif busses[i].get_time_equality() < busses[i-1].get_time_equality():
            busses[i].step += 1
            break

    if time_equalized:
        print("time equalized")
        break

print(busses[0].get_time())
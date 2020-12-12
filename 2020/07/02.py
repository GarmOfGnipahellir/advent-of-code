import re
import json

inp = open("./in01", mode="r").readlines()
inp = [line.strip() for line in inp]

first_pat = re.compile(r"(\w+ \w+) bags contain (.*).")
rule_pat = re.compile(r"(\d+) (\w+ \w+)")

ruledict = {}

for ln in inp:
    match = first_pat.match(ln)

    this_bag = match.group(1)

    ruledict[this_bag] = {}

    rules = match.group(2).split(", ")
    for rule in rules:
        match = rule_pat.match(rule)
        if match is None:
            continue

        num = int(match.group(1))
        color = match.group(2)
        
        ruledict[this_bag][color] = num

print(json.dumps(ruledict, indent=2))

def recursive_count(color, i):
    print("counting bags in", color)
    sm = 0
    for c, n in ruledict[color].items():
        print(n, c)
        sm += n + recursive_count(c, n)
        print("sum is", sm)
    return sm * i

n = recursive_count("shiny gold", 1)

print(n)
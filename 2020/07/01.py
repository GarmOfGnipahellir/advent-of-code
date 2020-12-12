import re

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

def recursive_search(color, i):
    # print("  " * i + "searching", color)
    if color not in ruledict.keys():
        print("  " * i + "no", color, "in rules")
        return False
    
    rules = ruledict[color]
    for other_color in rules.keys():
        if other_color == "shiny gold":
            print("  " * (i+1) + "found shiny gold in", color)
            return True
        else:
            if recursive_search(other_color, i + 1):
                return True
    
    return False

n = 0
for bag in ruledict.keys():
    if recursive_search(bag, 0):
        n += 1

print(n)
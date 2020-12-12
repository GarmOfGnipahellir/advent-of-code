import re

inp = open("./in01", mode="r").readlines()
inp = [line.strip() for line in inp]

passports = []

passport = ""
for line in inp:
    if line == "":
        passports.append(passport.lstrip())
        passport = ""
    else:
        passport += " " + line
passports.append(passport.lstrip())

def is_between(x, mn, mx):
    return int(x) >= mn and int(x) <= mx

hgt_pattern = re.compile(r"(\d+)(\w+)")
def hgt_checker(x):
    match = hgt_pattern.match(x)
    if match.group(2) == "cm":
        return is_between(match.group(1), 150, 193)
    else:
        return is_between(match.group(1), 59, 76)

hex_pattern = re.compile(r"#[a-f0-9]{6}")
pid_pattern = re.compile(r"\d{9}$")
field_checkers = {
    "byr": lambda x: is_between(x, 1920, 2002),
    "iyr": lambda x: is_between(x, 2010, 2020),
    "eyr": lambda x: is_between(x, 2020, 2030),
    "hgt": hgt_checker,
    "hcl": lambda x: hex_pattern.match(x) is not None,
    "ecl": lambda x: x in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"],
    "pid": lambda x: pid_pattern.match(x) is not None,
    # "cid": lambda x: False,
}
kvp_pattern = re.compile(r"(\w+):(.+)")

n = 0
for passport in passports:
    entries = passport.split()
    
    entrydict = {}
    for entry in entries:
        match = kvp_pattern.match(entry)
        k = match.group(1)
        v = match.group(2)
        entrydict[k] = v
    
    valid = True
    for name, checker in field_checkers.items():
        if name == "cid":
            continue

        if name not in entrydict.keys():
            valid = False
            break
        
        check = checker(entrydict[name])
        if name == "pid":
            print(f"{name}:{entrydict[name]}", "is", check)
        if not check:
            valid = False

    if valid:
        n += 1

print(n)
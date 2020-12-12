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

required_fields = [
    "byr",
    "iyr",
    "eyr",
    "hgt",
    "hcl",
    "ecl",
    "pid",
    # "cid",
]
field_pattern = re.compile(r"(\w+):")

n = 0
for passport in passports:
    matches = field_pattern.findall(passport)
    valid = True
    for field in required_fields:
        if field == "cid":
            continue

        if field not in matches:
            valid = False
            break
    
    if valid:
        n += 1

print(n)
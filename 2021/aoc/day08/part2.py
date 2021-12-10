# Advent of Code - Day 8 - Part Two

class Digit:
    chars: str

    def __init__(self, chars: str) -> None:
        self.chars = chars

    def __str__(self) -> str:
        return self.chars

    def __repr__(self) -> str:
        return f"Digit({self.chars})"

    def __len__(self) -> int:
        return len(self.chars)

    def __eq__(self, other) -> bool:
        if type(self) != type(other) or len(self) != len(other):
            return False
        for chr in self.chars:
            if chr not in other.chars:
                return False
        return True

    def __sub__(self, other):
        tmp = self.chars
        for ch in other.chars:
            tmp = tmp.replace(ch, "")
        return Digit(tmp)


def result(input):
    input = [(*[spl.split() for spl in ln.split("|")],) for ln in input]

    res = 0
    for entry in input:
        digits = [None for _ in range(10)]

        # get known digits
        for chars in entry[0]:
            l = len(chars)
            if l == 2:
                digits[1] = Digit(chars)
            elif l == 3:
                digits[7] = Digit(chars)
            elif l == 4:
                digits[4] = Digit(chars)
            elif l == 7:
                digits[8] = Digit(chars)

        l5 = list(filter(lambda chars: len(chars) == 5, entry[0]))
        for chars in l5:
            digit = Digit(chars)
            if len(digit - digits[1]) == 3:
                digits[3] = digit
            elif len(digit - digits[4]) == 3:
                digits[2] = digit
            else:
                digits[5] = digit

        l6 = list(filter(lambda chars: len(chars) == 6, entry[0]))
        for chars in l6:
            digit = Digit(chars)
            if len(digit - digits[1]) == 5:
                digits[6] = digit
            elif len(digit - digits[4]) == 2:
                digits[9] = digit
            else:
                digits[0] = digit

        num = ""
        for chars in entry[1]:
            for i, digit in enumerate(digits):
                if Digit(chars) == digit:
                    num += str(i)
        res += int(num)

    return res

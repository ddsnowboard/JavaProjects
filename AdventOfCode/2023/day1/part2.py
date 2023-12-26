import sys

def starting_number(s):
    numbers = dict(**{
        "one": 1,
        "two": 2,
        "three": 3,
        "four": 4,
        "five": 5,
        "six": 6,
        "seven": 7,
        "eight": 8,
        "nine": 9
    }, **{str(n): n for n in range(10)})
    for prefix, number in numbers.items():
        if s.startswith(prefix):
            return number
    return None

def calibration_number(line):
    digits = []
    curr_string = line
    while curr_string:
        n = starting_number(curr_string)
        if n:
            digits.append(n)
        curr_string = curr_string[1:]
    tens = digits[0]
    ones = digits[-1]
    out = 10 * tens + ones
    return out

print(sum(calibration_number(line) for line in sys.stdin))

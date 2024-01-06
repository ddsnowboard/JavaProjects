import sys

def calibration_number(line):
    digits = [int(c) for c in line if c.isdigit()]
    tens = digits[0]
    ones = digits[-1]
    return 10 * tens + ones

print(sum(calibration_number(line) for line in sys.stdin))

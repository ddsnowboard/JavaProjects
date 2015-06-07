from fractions import Fraction
from WillsLib import PrimeFactorizer
from functools import reduce
from re import compile
factorizer = PrimeFactorizer()
def floyd(number):
    """
    Floyd's Cycle Finding algorithm
    """
    re = compile(r"0[.](?P<tail>[0-9]+)")
    s = re.match(str(number)).group("tail")
    tortoise = 0
    hare = 1
    while s[tortoise] != s[hare]:
        tortoise += 1
        hare += 2
    length = hare - tortoise
    tortoise = 0
    hare = length
    
def findCycleLength(fraction):
    global factorizer
    # If the number doesn't repeat...
    if reduce([i == 5 or i == 3 for i in factorizer.factorize(fraction.denominator)], lambda x, y: x and y):
        return 0
    current_length = 1
    return floyd(float(fraction))

from fractions import Fraction
from decimal import Decimal, getcontext
from WillsLib import PrimeFactorizer
from functools import reduce
from pprint import pprint
from re import compile
factorizer = PrimeFactorizer()
# There is something with a cycle over 500 long, so 1000 isn't even enough.
# This works alright though.
getcontext().prec = 5000
def floyd(number):
    """
    Floyd's Cycle Finding algorithm
    """
    re = compile(r"0[.](?P<tail>[0-9]+)")
    s = re.match(str(number)).group("tail")
    tortoise = 2
    hare = 3
    # This checking here is effective, but a little bit dirty, seeing as it is
    # theoretically possible that a number might exist that would pass this but
    # lock up the function. But it works for 1000.
    while s[tortoise] != s[hare] or s[tortoise - 1] != s[hare - 1] or s[tortoise - 2] != s[hare - 2]:
        tortoise += 1
        hare += 2
    length = hare - tortoise
    tortoise = 0
    return length
def findCycleLength(fraction):
    global factorizer
    # If the number doesn't repeat... (ie, the prime factorization of the denominator
    # only contains the numbers 2 and 5, cf http://www.onemathematicalcat.org/algebra_book/online_problems/finite_or_inf_rep.htm)
    if reduce(lambda x, y: x and y, [i == 5 or i == 2 for i in factorizer.factorize(fraction.denominator)]):
        return 0
    current_length = 1
    return floyd(Decimal(fraction.numerator) / Decimal(fraction.denominator))



print(max([(i, findCycleLength(Fraction(1, i))) for i in range(2, 1001)], key=lambda x: x[1]))

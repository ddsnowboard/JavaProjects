from fractions import Fraction
from decimal import Decimal, getcontext
from WillsLib import PrimeFactorizer
from functools import reduce
from re import compile
factorizer = PrimeFactorizer()
getcontext().prec = 300
def floyd(number):
    """
    Floyd's Cycle Finding algorithm
    """
    re = compile(r"0[.](?P<tail>[0-9]+)")
    s = re.match(str(number)).group("tail")
    tortoise = 1
    hare = 2
    print("The tail is {}".format(s))
    while s[tortoise] != s[hare]:
        print(s[tortoise], s[hare])
        tortoise += 1
        hare += 2
    length = hare - tortoise
    tortoise = 0
    hare = length
    while s[tortoise] != s[hare]:
        tortoise += 1
        hare += 1
    mu = tortoise
    hare = tortoise
    while s[tortoise] != s[hare]:
        hare += 1
    print(hare - tortoise)
    return hare - tortoise
def findCycleLength(fraction):
    global factorizer
    print("The number is {}".format(fraction))
    # If the number doesn't repeat... (ie, the prime factorization of the denominator
    # only contains the numbers 2 and 5, cf http://www.onemathematicalcat.org/algebra_book/online_problems/finite_or_inf_rep.htm)
    if reduce(lambda x, y: x and y, [i == 5 or i == 2 for i in factorizer.factorize(fraction.denominator)]):
        return 0
    current_length = 1
    return floyd(Decimal(fraction.numerator) / Decimal(fraction.denominator))
print(max([(i, findCycleLength(Fraction(1, i))) for i in range(2, 1001)], key=lambda x: x[1]))

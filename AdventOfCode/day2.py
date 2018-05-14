from sys import stdin
from itertools import permutations
def getDivisiblePair(l):
    def divisible(a, b):
        if a < b:
            return divisible(b, a)
        return a % b == 0
    return list(filter(lambda x: divisible(*x) and x[0] > x[1], permutations(l, 2)))[0]

lines = map(lambda x: x.strip(), stdin)
digits = map(lambda x: x.split(), lines)
numbers = map(lambda x: map(lambda y: int(y), x), digits)
pairs = map(getDivisiblePair, numbers)
quotients = map(lambda x: x[0] // x[1], pairs)
print(sum(quotients))
"""
mins = map(lambda x: min(x), numbers)
maxes = map(lambda x: max(x), numbers)
differences = map(lambda x: x[0] - x[1], zip(maxes, mins))
print(sum(differences))
"""

from itertools import cycle, chain, count, islice
from fractions import Fraction

def fraction(start, sequence, depth):
    if depth == 0:
        return start
    else:
        next_denominator = next(sequence)
        return start + (1 / fraction(next_denominator, sequence, depth - 1))

def two(d):
    return fraction(Fraction(1), map(Fraction, cycle([2])), d)

def twenty_three(d):
    return fraction(Fraction(4), map(Fraction, cycle([1,3,1,8])), d)

def e(d):
    denominators = chain.from_iterable([1, 2 * n, 1] for n in count(1))
    return fraction(Fraction(2), map(Fraction, denominators), d)

def sum_digits_in_numerator(n):
    digits = str(n.numerator)
    return sum(int(i) for i in digits)

print(sum_digits_in_numerator(e(99)))

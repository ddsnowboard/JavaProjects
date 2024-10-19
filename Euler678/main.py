from itertools import product, count
from functools import partial
from math import log, isclose
# Damn this one is hard

def all_tuples(N):
    min_e = 2
    min_f = 3
    for b in count(1):
        if b ** min_e > N:
            break
        for a in range(1, b):
            if b ** min_e + a ** min_e > N:
                break
            for e in count(2):
                left_side = b ** e + a ** e
                if left_side > N:
                    break
                for f in count(3):
                    # can we find an integral c?
                    potential_c = left_side ** (1 / f)
                    if isclose(potential_c, round(potential_c)):
                        yield (a, b, round(potential_c), e, f)
                    elif potential_c < 2.0:
                        break

def integral_log_or_nothing(x, base):
    if x < 0 or base < 0:
        raise Exception("You can't do negative numbers")

    n_steps = 1
    while x != 1:
        if x % base != 0:
            return None
        x //= base
        n_steps += 1
    return n_steps

def all_tuples(N):
    min_e = 2
    min_f = 3
    for b in count(2):
        # minimum e is 2, so this is the same as b ** min_e, but a bit faster
        if (b * b) + 1 > N:
            break
        for a in range(1, b):
            if b ** min_e + a ** min_e > N:
                break
            for e in count(2):
                left_side = b ** e + a ** e
                if left_side > N:
                    break
                for c in range(2, round(left_side ** (1/3)) + 1):
                    if left_side % c != 0:
                        continue
                    potential_f = integral_log_or_nothing(left_side, c)
                    if potential_f is not None:
                        yield (a, b, c, e, potential_f)

                    """
                    potential_f = log(left_side) / log(c)
                    # yield (a, b, c, e, round(potential_f))
                    if isclose(potential_f, round(potential_f)) and round(potential_f) >= 3:
                        yield (a, b, c, e, round(potential_f))
                    """


print(sum(1 for _ in all_tuples(10 ** 7)))

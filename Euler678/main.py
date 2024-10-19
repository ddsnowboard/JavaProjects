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
                if b ** e + a ** e > N:
                    break
                for f in count(3):
                    # can we find an integral c?
                    potential_c = (a ** e + b ** e) ** (1 / f)
                    if isclose(potential_c, round(potential_c)):
                        yield (a, b, round(potential_c), e, f)
                    elif potential_c < 2.0:
                        break




print(len(list(all_tuples(10 ** 7))))

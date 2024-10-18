from itertools import product
from functools import partial
# Damn this one is hard

def good(N, t):
    (a, b, c, e, f) = t
    if a >= b:
        return False
    if e < 2:
        return False
    if f < 3:
        return False
    if c ** f > N:
        return False

    if a ** e + b ** e == c ** f:
        return True
    else:
        return False

def all_tuples(N):
    everything = product(range(1, N), range(1, N), range(1, N), range(1, N), range(1, N))
    return filter(partial(good, N), everything)

print(next(all_tuples(28)))

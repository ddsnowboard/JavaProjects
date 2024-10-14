from itertools import permutations, chain

def pandigitials(n):
    digits = list(range(1, n + 1))
    orderings = permutations(digits)
    def combine(l):
        return int("".join(str(d) for d in l))
    return [combine(l) for l in orderings]

def is_prime(n):
    current_factor = 2
    while current_factor < n ** 0.5:
        if n % current_factor == 0:
            return False
        current_factor += 1
    return True

all_pandigitals = chain.from_iterable(pandigitials(n) for n in range(1, 10))
print(max(filter(is_prime, all_pandigitals)))

from itertools import combinations, permutations, starmap
from concurrent.futures import ProcessPoolExecutor

with open("somePrimes.txt") as f:
    primes = {int(l.strip()) for l in f if l}

def is_prime_pair_set(s):
    def is_concat_prime(l, r):
        concat = int(str(l) + str(r))
        return concat in primes
    all_pairs = permutations(s, 2)
    return all(starmap(is_concat_prime, all_pairs))

# pool = ProcessPoolExecutor(4)

all_sets = combinations((p for p in primes if p <= 1000), 4)
count = 0
def ticker(x):
    global count
    count += 1
    if count % 100_000 == 0:
        pass
        # print("BING {}".format(count))
    return x
all_sets = map(ticker, all_sets)
print([s for s in all_sets if is_prime_pair_set(s)])

from sys import argv
max_n = int(argv[1])
primes = [True] * max_n
# 1 is not prime
primes[0] = False

def find_next_prime(current_prime):
    try:
        return next(current_prime + idx + 1 for (idx, is_prime) in enumerate(primes[current_prime:]) if is_prime)
    except StopIteration:
        return None

current_prime = 2
current_idx = current_prime * current_prime
while True:
    while current_idx <= max_n:
        primes[current_idx - 1] = False
        current_idx += current_prime
    current_prime = find_next_prime(current_prime)
    if not current_prime:
        break
    current_idx = current_prime * current_prime
for (idx, p) in enumerate(primes):
    if p:
        print(idx + 1)

from functools import reduce
def reverse_number(n):
    out = 0
    while n > 0:
        out *= 10
        out += n % 10
        n //= 10
    return out

def is_lychrel(n):
    orig = n
    for _ in range(50):
        n += reverse_number(n)
        if n == reverse_number(n):
            return False
    return True

assert reverse_number(12345) == 54321
assert reverse_number(1111) == 1111
assert reverse_number(0) == 0

assert not is_lychrel(47)
assert is_lychrel(196)
assert not is_lychrel(349)
assert is_lychrel(4994)

print(reduce(lambda x, y: x + (1 if is_lychrel(y) else 0), range(10, 10000), 0))

import cProfile

from math import log10 as log, floor
def is_palindrome_bin(i):
    s = "{0:b}".format(i)
    return s == s[::-1]

def is_palindrome_dec(i):
    s = str(i)
    return s == s[::-1]

def is_palindrome(i):
    return is_palindrome_dec(i) and is_palindrome_bin(i)

def main():
    print(sum(i for i in range(1, 1000000) if is_palindrome(i)))

main()

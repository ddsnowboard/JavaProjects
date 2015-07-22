from math import sqrt, fabs
import profile
class Primes:
    def __init__(self, init = 0):
        self.primes = {2 : True}
        curr_number = 3
        while len({i : j for i, j in self.primes.items() if j}) < init:
           self.primes[curr_number] = Primes.checkPrimality(curr_number)
           curr_number += 1
    def isPrime(self, number):
        out = self.primes.get(number, None)
        if out is None:
            self.primes[number] = Primes.checkPrimality(number)
            return self.isPrime(number)
        return out
    @classmethod
    def checkPrimality(self, number):
        number = fabs(number)
        if number == 2:
            return True
        for i in range(2, int(sqrt(number) + 1)):
            if number % i == 0:
                return False
        return True
def main():
    p = Primes(100)
    highestNumber = 0
    highestProduct = 0
    for i in range(-999, 1000):
        if i % 100 == 0:
            print("i = {}".format(i))
        for j in range(-999, 1000):
            currentNumber = 0
            currentOutput = currentNumber ** 2 + i * currentNumber + j
            while p.isPrime(currentOutput):
                currentNumber += 1
                currentOutput = currentNumber ** 2 + i * currentNumber + j
            if currentNumber > highestNumber: 
                highestNumber = currentNumber
                highestProduct = i * j
    print("The highest product was {}".format(highestProduct))
# profile.run("main()")
main()

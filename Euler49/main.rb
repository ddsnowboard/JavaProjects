# The arithmetic sequence, 1487, 4817, 8147, in which each of the terms increases by 3330, is unusual in two ways: (i) each of the three terms are prime, and, (ii) each of the 4-digit numbers are permutations of one another.
#
# There are no arithmetic sequences made up of three 1-, 2-, or 3-digit primes, exhibiting this property, but there is one other 4-digit increasing sequence.
#
# What 12-digit number do you form by concatenating the three terms in this sequence?

class PrimeMachine
  @@primes = [2, 3, 5, 7]
  def initialize
    @index = 0
  end
  def next_prime
    if @@primes[@index]
      @index += 1
      return @@primes[@index - 1]
    else
      next_prime = @@primes[-1] + 2
      while not is_prime next_prime
        next_prime += 1
      end
      @@primes.push(next_prime)
      @index += 1
      return next_prime
    end
  end

  def is_prime(n)
    # You better have already generated all the prime numbers
    # below this one, or else this function won't work.
    if @@primes.index(n) != nil
      return true
    end
    for i in @@primes
      if n % i == 0
        return false
      elsif i > (n / 2)
        return true
      end
    end
    return true
  end
end

def is_permutation(a, b)
  nA = 0
  nB = 0
  a.to_s.each_byte do |l|
    nA = nA | (2 ** (l - '0'.ord))
  end
  b.to_s.each_byte do |l|
    nB = nB | (2 ** (l - '0'.ord))
  end
  return nA == nB
end

pm = PrimeMachine.new
n = pm.next_prime
while n < 10000
  n = pm.next_prime
end

pm = PrimeMachine.new
n = 0
while (n = pm.next_prime) < 10000
  if n < 1000
    next
  else
    if pm.is_prime(n)
      for i in 1...((10000 - n) / 2)
        if pm.is_prime(n + i) and pm.is_prime(n + 2 * i)
          if is_permutation(n, n + i) and is_permutation(n, n + 2 * i)
            puts "#{n} #{(n + i)} #{(n + 2 * i)}"
          end
        end
      end
    end
  end
end

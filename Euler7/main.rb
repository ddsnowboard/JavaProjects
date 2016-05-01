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

    private
    def is_prime(n)
        # You better have already generated all the prime numbers
        # below this one, or else this function won't work.
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

pm = PrimeMachine.new
for i in 1...10001
    pm.next_prime
end

puts pm.next_prime

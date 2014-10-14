import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
// Dear future me, 
// This takes a long time and it always kicks me an OutOfMemoryError, so I think it might work in theory
// but I can't get it to actually work. Maybe on my faster computer. O well. 

public class Euler3 {

    public static void main(String[] args) {
        long num = 600851475143l;
        List<Long> primes = primeFactor(num);
        Collections.sort(primes);
        System.out.println(primes.get(primes.size() - 1));
    }

    public static List<Long> primeFactor(long j) {

        List<Long> primes = new ArrayList<>();
        boolean prime;
        if (j % 2 == 0) {
            primes.add(2l);
            primes.addAll(primeFactor(j / 2l));
            return primes;
        } else if (j % 3 == 0) {
            primes.add(3l);
            primes.addAll(primeFactor(j / 3l));
            return primes;
        } else {
            for (long i = 5; i <= j; i += 2) {
                prime = true;
                for (long p = 2; p <= Math.sqrt(i) + 1; p++) {
                    if (i % p == 0) {
                        prime = false;
                        break;
                    }
                }
                if (prime && j % i == 0) {
                    primes.add(i);
                    primes.addAll(primeFactor(j / i));
                    return primes;
                }
            }
            primes.add(j);
            return primes;
        }
    }

}

import java.util.HashMap;
import java.util.ArrayList;
import java.lang.Integer;
import java.lang.Boolean;
import java.util.Map;


class Primes {
    public HashMap<Integer, Boolean> primes;
    public Primes(int start)
    {
        primes = new HashMap<>();
        primes.put(2, Boolean.TRUE);
        int currentPrime = 3;
        boolean prime = true;
        while(numberOfPrimes() < start)
        {
            primes.put(currentPrime, checkPrimality(currentPrime));
            currentPrime++;
        }
    }
    public Primes()
    {
        this(0);
    }
    public boolean isPrime(Integer number)
    {
        number = Math.abs(number);
        if(!primes.containsKey(number))
        {
            primes.put(number, checkPrimality(number));
        }
        return primes.get(number);
    }
    public int numberOfPrimes()
    {
        int out = 0;
        for(Map.Entry e : primes.entrySet())
        {
            if(e.getValue() == Boolean.TRUE)
            {
                out++;
            }
        }
        return out;
    }
    private Boolean checkPrimality(Integer i)
    {
        for(int o = 2;o < (int)(i / 2 + 1);o++)
        {
            if(i % o == 0)
            {
                return Boolean.FALSE;
            }
        }
        return Boolean.TRUE;
    }
}

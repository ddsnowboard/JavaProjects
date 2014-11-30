/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package euler21;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.Map;

/**
 *
 * @author ddsnowboard
 */
public class DivisorAdder {

    private final Integer[] fastPrimes;
    private final int MAX_NUMBER;
    private HashMap<Integer, Integer> cache = new HashMap<>();

    public DivisorAdder(int maximum) {
        MAX_NUMBER = maximum;
        fastPrimes = Utility.sieve(MAX_NUMBER);
    }

    public int sumDivisors(int input) {
        int out;
        if (cache.get(input) == null) {
            ArrayList<Integer> primeFactors = Utility.factorize(fastPrimes, input);
            ArrayList<ArrayList<Boolean>> combinations = Utility.allCombinations(primeFactors.size());
            Map<Integer, Boolean> divisors = new HashMap<>();
            // One is always a divisor. 
            divisors.put(1, true);
            Integer curr;
            int factorAmt = primeFactors.size();
            for (ArrayList<Boolean> l : combinations) {
                curr = 1;
                for (int i = 0; i < factorAmt; i++) {
                    if (l.get(i)) {
                        curr *= primeFactors.get(i);
                    }
                }
                divisors.put(curr, true);
            }
            // Use Utility.allCombinations to get all the combinations of the prime 
            // factors. That will give you all the divisors. 
            out = Utility.sum(divisors.keySet());
            cache.put(input, out);
            return out;
        } else {
            return cache.get(input);
        }
    }
}

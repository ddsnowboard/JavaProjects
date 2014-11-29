/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package euler21;

import java.util.ArrayList;

/**
 *
 * @author ddsnowboard
 */
public class DivisorAdder {
    private final Integer[] fastPrimes;
    private final int MAX_NUMBER;

    public DivisorAdder(int maximum) {
        MAX_NUMBER = maximum;
        fastPrimes = Utility.sieve(MAX_NUMBER);
    }

//    public int sumDivisors(int input) {
//        ArrayList<Integer> possibleFactors = new ArrayList<>();
//        ArrayList<Integer> primeFactors = Utility.factorize(fastPrimes, input);
//        // Use Utility.allCombinations to get all the combinations of the prime 
//        // factors. That will give you all the divisors. 
//    }
}

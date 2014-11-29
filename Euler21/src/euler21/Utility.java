/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package euler21;

import java.util.ArrayList;
import java.util.Set;

/**
 *
 * @author ddsnowboard
 */
public class Utility {

    public static Integer[] sieve(int max) {
        ArrayList<Integer> output = new ArrayList<>();
        Integer current;
        for (int i = 2; i <= max; i++) {
            output.add(i);
        }
        for (int i = 2; i < max; i++) {
            if (output.contains(i)) {
                current = i * 2;
                while (current < max) {
                    output.remove(current);
                    current += i;
                }
            }
        }
        return output.toArray(new Integer[output.size()]);
    }

    public static ArrayList<Integer> factorize(Integer[] primes, int number) {
        ArrayList<Integer> output = new ArrayList<>();
        int orig = number;
        while (number != 1) {
            for (Integer i : primes) {
                if (number % i == 0) {
                    try {
                        output.add(i);
                        number /= i;
                        break;
                    } catch (OutOfMemoryError e) {
                        System.out.printf("Orig was %d; number was %d", orig, number);
                        throw e;
                    }
                }
            }
        }
        return output;
    }

    public static ArrayList<ArrayList<Boolean>> allCombinations(int size) {
        double num = Math.pow(2, size) - 1;
        ArrayList<String> binaryStrings = new ArrayList<>();
        ArrayList<ArrayList<Boolean>> output = new ArrayList<>();
        StringBuilder currString;
        for (int i = 1; i < num; i++) {
            currString = new StringBuilder(Integer.toBinaryString(i));
            while (currString.length() < size) {
                currString.insert(0, "0");
            }
            binaryStrings.add(currString.toString());
        }
        for (String s : binaryStrings) {
            output.add(new ArrayList<Boolean>());
            for (int i = 0; i < s.length(); i++) {
                output.get(output.size() - 1).add((s.charAt(i) == '1'));
            }
        }
        return output;
        // So, I need to find a way to get all the combinations, and to do that I'm 
        // going to get the binary string representations of all the numbers between one and 
        // 2^(list.size()+1)-1), which will give me every possible permutation
        // of one and that is the length of the size of the list. Then I'll iterate
        // through each of them and use them to return the list of lists of booleans
        // (not yet denoted) that I can use elsewhere. So if I put in 2, it should 
        // give me every possible set of one and zero that is 2 long (00, 10, 01, 11)
        // Explanation of method: https://math.stackexchange.com/questions/141302/how-to-find-all-possible-combinations-of-a-set-of-options
        // Help on printing binary numbers (they'll need to be zero-padded):https://stackoverflow.com/questions/4421400/how-to-get-0-padded-binary-representation-of-an-integer-in-java
        // https://stackoverflow.com/questions/5263187/print-an-integer-in-binary-format-in-java
    }

    public static int sum(Set<Integer> s) {
        int out = 0;
        for (Integer i : s) {
            out += i;
        }
        return out;
    }
}

/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package euler21;

/**
 *
 * @author ddsnowboard
 */
public class Euler21 {

    public static final int MAX_NUMBER = 10000;

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        DivisorAdder divisorAdder = new DivisorAdder(Euler21.MAX_NUMBER);
        System.out.println(Utility.factorize(Utility.sieve(100),55));
    }

}

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

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        DivisorAdder divisorAdder = new DivisorAdder();
        int end = 0;
//        for (int i = 0; i < 10000; i++) {
//            for (int j = 0; j < 10000; j++) {
        int i = 220;
        int j = 284;
        System.out.printf("%d, %d", divisorAdder.sumDivisors(j), divisorAdder.sumDivisors(i));
                if (divisorAdder.sumDivisors(i) == j && divisorAdder.sumDivisors(j) == i) {
                    end += i;
                    end += j;
                }
//            }
//        }
        System.out.println(end);
    }

}

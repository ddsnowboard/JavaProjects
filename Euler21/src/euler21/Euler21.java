
package euler21;

import java.util.HashMap;

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
        HashMap<Integer, Boolean> amici = new HashMap<>();
        int out = 0;
        for (int i = 0; i < MAX_NUMBER; i++) {
            for (int j = 0; j < MAX_NUMBER; j++) {
                if (divisorAdder.sumDivisors(i) == j && divisorAdder.sumDivisors(j) == i && i != j) {
                    amici.put(j, Boolean.TRUE);
                    amici.put(i, Boolean.TRUE);
                }
            }
        }
        for(Integer i : amici.keySet())
        {
            out+=i;
        }
        System.out.println(out);
    }

}

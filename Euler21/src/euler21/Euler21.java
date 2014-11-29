package euler21;

/**
 *
 * @author ddsnowboard
 */
public class Euler21 {

    public static final int MAX_NUMBER = 250;

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        DivisorAdder divisorAdder = new DivisorAdder(Euler21.MAX_NUMBER);
        int out = 0;
        for (int i = 0; i < MAX_NUMBER; i++) {
            for (int j = 0; j < MAX_NUMBER; j++) {
                if (divisorAdder.sumDivisors(i) == j && divisorAdder.sumDivisors(j) == i) {
                    out += (i + j);
                }
            }
        }
        System.out.println(out / 2);
    }

}

package euler12;

import java.util.ArrayList;

/**
 *
 * @author ddsnowboard
 */
public class Euler12 {

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        long index = 1;
        long thisTriangle = 1;
        int counter = 0;
        while (factors(thisTriangle) < 500) {
            index++;
            thisTriangle += index;
            if(counter==100)
            {
                counter = 0;
            }
            counter++;
        }
        System.out.println(thisTriangle);
    }

    public static int factors(long n) {
        ArrayList<Long> factors = new ArrayList<>();
        for (long i = 1; i <= n / 2; i++) {
            if(factors.contains(i) == true)
                break;
            else if (n % i == 0) {
                factors.add(i);
                factors.add(n/i);
            }
        }
        return factors.size();
    }
}

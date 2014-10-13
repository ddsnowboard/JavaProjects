package euler12;

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
        long thisTriangle = triangle(index);
        while (factors(thisTriangle) < 500) {
            index++;
            thisTriangle = triangle(index);
        }
        System.out.println(thisTriangle);
    }

    public static int factors(long n) {
        int out = 0;
        for (long i = 1; i <= n / 2; i++) {
            if (n % i == 0) {
                out++;
            }
        }
        out++; // For the number itself
        return out;
    }

    public static int triangle(long index) {
        int out = 0;
        for (int i = 0; i <= index; i++) {
            out += i;
        }
        return out;
    }
}

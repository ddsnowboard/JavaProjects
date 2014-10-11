package euler10;

import java.util.ArrayList;

/**
 *
 * @author ddsnowboard
 */
public class Euler10 {

    public static void main(String[] args) {
        ArrayList<Integer> list = primesBelow(2000000);
        System.out.println(list.size());
        System.out.println(sum(list));
    }
    public static long sum(ArrayList<Integer> a)
    {
        long out = 0;
        for(Integer i : a)
            out+=i;
        return out;
    }

    public static ArrayList<Integer> primesBelow(int top) {
        ArrayList<Integer> out = new ArrayList<>();
        boolean prime;
        out.add(2);
        for (int i = 3; i <= top; i+=2) {
            prime = true;
            for (int j = 2; j <= Math.sqrt(i); j++) {
                if (i % j == 0) {

                    prime = false;
                    break;
                }
            }
            if (prime) {
                out.add(i);
            }
        }
        return out;
    }

}

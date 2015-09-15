import java.math.BigInteger;


public class Euler20 {

    public static void main(String[] args) {
        BigInteger factorial = fact(100);
        System.out.println(factorial);
        String factString = String.valueOf(factorial);
        int total = 0;
        for(int i = 0;i<factString.length();i++)
        {
            total += Integer.parseInt(String.valueOf(factString.charAt(i)));
        }
        System.out.println(total);
    }

    private static BigInteger fact(int i) {
        BigInteger end = new BigInteger("1");
        for(int p = i;p > 0;p--)
        {
            end = end.multiply(BigInteger.valueOf(p));
        }
        return end;
    }

}

public class Palindrome {
    public static void main(String[] args) throws NumberFormatException {
        System.out.printf("%b is false\n", isPalindrome(Integer.MAX_VALUE));
        System.out.printf("%b is true\n", isPalindrome(101101));
        System.out.printf("%b is true\n", isPalindrome(110011));
        System.out.printf("%b is false\n", isPalindrome(1100110));
        int i = Integer.decode(args[0]);
        System.out.println(isPalindrome(i));
    }

    public static boolean isPalindrome(int i)
    {
        return isPalindrome(i, (int) Math.log10(i) + 1);
    }

    public static boolean isPalindrome(int i, int length) {
        if(i == 0)
            return true;
        else if(i < 0)
            return false;
        int lastDigit = i % 10;
        int firstDigit = i / (int) Math.pow(10, length - 1);
        // System.out.printf("The first digit is %d and the last is %d%n", firstDigit, lastDigit);
        if(lastDigit == firstDigit) {
            i %= Math.pow(10, length - 1);
            i /= 10;
            // We chopped off 2 numbers, so it needs to be 2 less
            return isPalindrome(i, length - 2);
        }
        else {
            return false;
        }
    }

}

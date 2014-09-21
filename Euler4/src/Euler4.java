import java.util.ArrayList;
import java.util.Collections;


public class Euler4 {

	public static void main(String[] args) {
		ArrayList<Long> palindromes = new ArrayList<Long>();
		for(long i = 999;i>=100;i--)
		{
			for(long j = 999; j>=100;j--)
			{
				if(isPalindrome(i*j))
				{
					System.out.printf("%d, %d%n", i, j);
					System.out.println(i*j);
					palindromes.add(i*j);
				}
			}
		}
		Collections.sort(palindromes);
		System.out.println(palindromes.get(palindromes.size()-1));

	}
	public static boolean isPalindrome(long n)
	{
		String reverse = new StringBuilder(String.valueOf(n)).reverse().toString();
		return reverse.equals(String.valueOf(n));
	}

}

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;


public class Euler3 {

	public static void main(String[] args) {
		long num=600851475143l;
		List <Integer> primes = sieve((long)(num/2)+1);
		Collections.reverse(primes);
		for (int i : primes)
		{
			if (i%num == 0)
			{
				System.out.print(i);
				return;
			}
		}
	}
	public static List<Integer> sieve(long j)
	{
		List <Integer>primes = new ArrayList<Integer>();
		int p;
		for(int i=2;i<=j;i++)
		{
			primes.add(i);
		}
		for (int i = 0;i<primes.size();i++)
		{
			
			p=primes.get(i);
			for(int q=2;q<j;q++)
			{
				if(primes.contains(p*q))
				{
					primes.remove((p*q));
				}
			}
		}
		return primes;
		
	}

}

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
// Dear future me, 
// This takes a long time and it always kicks me an OutOfMemoryError, so I think it might work in theory
// but I can't get it to actually work. Maybe on my faster computer. O well. 

public class Euler3 {

	public static void main(String[] args) {
		long num=600851475143l;
		List<Long> primes = sieve((long)(num/2)+1);
//		List<Long> primes = sieve((long)(10000));
		Collections.reverse(primes);
//		System.out.println(primes.get(0));
		for (long i : primes)
		{
			if (i%num == 0)
			{
				System.out.printf("The largest prime factor is %d", i);
				return;
			}
		}
	}
	public static List<Long> sieve(long j)
	{
		List<Long>primes = new ArrayList<Long>();
		System.out.println("Did something");
		primes.add(2l);
		for(long i=3;i<=j;i+=2)
		{
			primes.add(i);
		}
		long p = 0;
		List<Long> primeprime = primes;
		for (int i = 0;i<(long)(Math.sqrt(primes.size())+1);i++)
		{
			p=primes.get(i);
			for(int q=2;q<j;q++)
			{
				if(primeprime.contains(p*q))
				{
					primeprime.remove(primeprime.indexOf(p*q));
				}
			}
		}
		return primes;
		
	}

}

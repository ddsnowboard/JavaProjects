import java.util.ArrayList;
import java.util.List;


public class Euler3 {

	public static void main(String[] args) {
		long num=600851475143L;
		List <Integer> primes = sieve(num);
		List <Integer> factors = new ArrayList<Integer>();
//		for(int m=0;m<primes.size();m++)
//		{
//			int l=primes.get(m);
//			System.out.print(l);
//			System.out.print("\n");
//		}
		for(int i=0;i<primes.size();i++)
		{
			if((num % (primes.get(new Integer(i))))==0)
			{
				num= num/primes.get(new Integer(i));
				factors.add(primes.get(new Integer(i)));
				i=0;
			}
		}
		
		for (int i=0;i<factors.size();i++)
		{
			long k=factors.get(i);
			System.out.print(k);
			System.out.print("\n");
		}

	}
	public static List<Integer> sieve(Long num)
	{
		List <Integer>primes = new ArrayList<Integer>();
		int p;
		for(int i=2;i<=num;i++)
		{
			primes.add(i);
		}
		for (int i = 0;i<primes.size();i++)
		{
			p=primes.get(i);
			for(int q=2;q<num/2;q++)
			{
				if(primes.contains(p*q))
				{
					primes.remove((p*q));
				}
			}
		}
		for(int m=0;m<primes.size();m++)
		{
			System.out.print(primes.get(m));
			System.out.print("\n");
		}
		return primes;
		
	}

}

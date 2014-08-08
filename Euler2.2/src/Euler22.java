
import java.util.ArrayList;
import java.util.List;



public class Euler22 {

	
	public static void main(String[] args) {
		List<Integer> nums = Fib(4000000);
		int sum=0;
		for(int p=0;p<nums.size();p++)
		{
			if(nums.get(p)%2==0)
			{
				sum=sum+nums.get(p);
			}
		}
		System.out.println("--\n");
		System.out.print(sum);
	}
	public static List<Integer> Fib(int max)
	{
		List<Integer> fibs = new ArrayList<Integer>();
		int i=2;
		int n;
		fibs.add(1);
		fibs.add(2);
		do
		{
			n=fibs.get(i-1)+fibs.get(i-2);
			fibs.add(n);
			i++;
		}
		while(fibs.get(fibs.size()-1)<=max);
		fibs.remove(fibs.size()-1);
		for(int q=0;q<fibs.size();q++)
		{
			System.out.print("\n");
			System.out.print(fibs.get(q));
		}
		return fibs;
	}

}

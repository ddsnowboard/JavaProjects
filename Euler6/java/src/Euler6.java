
public class Euler6 {
	public static void main(String[] args)
	{
		int sumSquares = 0;
		for(int i = 1; i<=100;i++)
		{
			sumSquares+=i*i;
		}
		int squareSum = 0;
		for(int i = 1;i<=100;i++)
		{
			squareSum+=i;
		}
		squareSum = squareSum*squareSum;
		System.out.println(squareSum-sumSquares);
	}

}
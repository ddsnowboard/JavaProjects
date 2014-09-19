import java.math.BigInteger;


public class Euler16 {
	public static void main(String[] args)
	{
		BigInteger i = new BigInteger("2");
		i = i.pow(1000);
		String s = i.toString();
		int end = 0;
		for(int p = 0; p<s.length();p++)
		{
			end+=Integer.parseInt(String.valueOf(s.charAt(p)));
		}
		System.out.print(end);
	}
	
}

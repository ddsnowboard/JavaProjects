import java.util.ArrayList;
import java.util.List;
import java.awt.*;
import javax.swing.*;


public class main {

	/**
	 * @param args
	 */
	public static void main(String[] args) {
		/*get number from window*/
		final int number=9;
		int running=number;
		int test=2;
		ArrayList <Integer> factors=new ArrayList<Integer>();
		while(!(test >= number/2))
		{
			if((running%test == 0)&& !(running<=2))
			{
				factors.add(test);
				running=running/test;
				test=2;
			}
			else
			{
				test++;
				
			}
		}
		if(!(factors.size()==0))
		{
			for(int i=0;i<factors.size();i++)
			{
				System.out.println(factors.get(i));
			}
		}
		else
		{
			System.out.print(number);
			System.out.print(" is prime!");
		}

	}

}


public class main {

	/**
	 * @param args
	 */
	public static void main(String[] args) {
		boolean[] lockers;
		lockers = new boolean[1000];
		int current=0;
		int end=0;
		int l=0;
		for(int i=0;i<1000;i++)
		{
			lockers[i]=false;
		}
		for(int i=1;i<1000;i++)
		{
			while (l<1000)
			{
				
			}
		}

//		for(int q=1;q<1000;q++)
//		{
//			for(int p=1;p<1000;p++)
//			{
//				if(p%q==0)
//				{
//					if(lockers[p]==true)
//					{
//						lockers[p-1]=false;
//					}
//					else if(lockers[p]==true)
//					{
//						lockers[p-1]=false;
//					}
//				}
//			}
//		}
		for (int o=0;o<1000;o++)
		{
			if(lockers[o]==true)
			{
				end++;
				
			}
		}
		System.out.print(end);
	}

}

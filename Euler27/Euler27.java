import java.util.ArrayList;
import java.util.HashMap;
class Euler27
{
    public static void main(String[] argv)
    {
        Primes p = new Primes(30);
        ArrayList<Integer> highestList = new ArrayList<>();
        ArrayList<Integer> currentList;
        HashMap<String, Integer> nums = new HashMap<>();
        int highestProduct = 0;
        int highestNumber = 0;
        int currentNumber = 0;
        int currentOutput;
        for(int i = -999; i<=1000;i++)
        {
            for(int j = -999; j<=1000;j++)
            {
                currentList = new ArrayList<>();
                currentNumber = 0;
                currentOutput = (currentNumber * currentNumber) + (i * currentNumber) + j;
                while(p.isPrime(currentOutput))
                {
                    currentList.add(currentOutput);
                    currentNumber++;
                    currentOutput = (currentNumber * currentNumber) + (i * currentNumber) + j;
                }
                if(currentNumber > highestNumber)
                {
                    nums.put("i", i);
                    nums.put("j", j);
                    highestProduct = i * j;
                    highestNumber = currentNumber;
                    highestList = new ArrayList<>(currentList);
                }
            }
        }
        System.out.printf("The highest number is %d and the highest product is %d. i was %d and j was %d.%nThe numbers are:%n", highestNumber, highestProduct, nums.get("i"), nums.get("j"));
        for(int i : highestList)
        {
            System.out.println(i);
        }
    }
}

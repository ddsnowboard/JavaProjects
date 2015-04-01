import java.util.ArrayList;
import java.util.HashMap;

public class Euler47 {
// https://projecteuler.net/problem=47
	// I really don't understand the question. It says distinct prime factors, but they aren't all distinct. They have two like three times. I'm confused. 
	public static void main(String[] args) {
		Factorizer factorizer = new Factorizer();
		int currentFirstNumber = 2;
		final int LENGTH_OF_SEQUENCE = 3;
		int[] currentFour = new int[LENGTH_OF_SEQUENCE];
		HashMap<Integer, Boolean> currentFactors = new HashMap<>();
		boolean found = false;
		ArrayList<Integer> currentNumberFactors;
		while (!found) {
			for (int i = 0; i < LENGTH_OF_SEQUENCE; i++)
				currentFour[i] = currentFirstNumber + i;
			for (int i : currentFour) {
				currentNumberFactors = factorizer.primeFactors(i);
				for (int j : currentNumberFactors) {
					if (!currentFactors.containsKey(j)) {
						currentFactors.put(j, true);
						found = true;
					} else {
						found = false;
						System.out.println("Overwritten!");
						break;
					}
				}
			}
			currentFirstNumber++;
			currentFactors = new HashMap<Integer, Boolean>();
		}
		System.out.println(currentFirstNumber);
	}
}

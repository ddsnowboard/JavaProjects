import java.util.ArrayList;
import java.util.HashMap;

public class Euler47 {
// https://projecteuler.net/problem=47
	// I understand the question now. The thing they're asking is whether there are three numbers in a row that each 
	// themselves have three distinct prime factors, or four with four. Ok. Got it. 
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

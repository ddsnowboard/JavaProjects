import java.util.ArrayList;
import java.util.HashSet;

public class Euler47 {
	// https://projecteuler.net/problem=47
	public static void main(String[] args) {
		Factorizer factorizer = new Factorizer();
		int currentFirstNumber = 1;
		final int LENGTH_OF_SEQUENCE = 4;
		int[] currentFour = new int[LENGTH_OF_SEQUENCE];
		ArrayList<HashSet<Integer>> currentFourFactors;
		boolean found = false;
		ArrayList<Integer> currentNumberFactors;
		while (!found) {
			currentFirstNumber++;
			for (int i = 0; i < LENGTH_OF_SEQUENCE; i++)
				currentFour[i] = currentFirstNumber + i;
			currentFourFactors = new ArrayList<>();
			for (int i = 0; i < LENGTH_OF_SEQUENCE; i++) {
				currentNumberFactors = factorizer.primeFactors(currentFour[i]);
				currentFourFactors.add(new HashSet<>());
				for (int j : currentNumberFactors) {
					currentFourFactors.get(i).add(j);
				}
			}
			found = true;
			for (HashSet<Integer> s : currentFourFactors) {
				found = found && s.size() == LENGTH_OF_SEQUENCE;
			}
		}
		System.out.println(currentFirstNumber);
	}
}

import java.text.DecimalFormat;
import java.util.ArrayList;
import java.util.HashMap;

public class Euler24B {
	public static void main(String[] args) {
		ArrayList<Double> numbers = new ArrayList<>();
		final double GOAL = 5;
		double currentNumber = 012;
		int[] NUMBERS = { 0, 1, 2 };
		// final double GOAL = 1e6;
		// double currentNumber = 0123456789d;
		// int[] NUMBERS = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9 };
		HashMap<Character, Boolean> currSet;
		String currString;
		DecimalFormat df = new DecimalFormat(repeatString("0", NUMBERS.length));
		boolean good;
		while (numbers.size() < GOAL) {
			currSet = new HashMap<>();
			currString = df.format(currentNumber);
			for (int i = 0; i < currString.length(); i++) {
				currSet.put(currString.charAt(i), Boolean.TRUE);
			}
			good = true;
			for (int p = 0; p < currString.length(); p++) {

			}
			if (currSet.con() == NUMBERS.length) {
				numbers.add(currentNumber);
				if (numbers.size() % 1000 == 0)
					System.out.printf("Found %d%n", numbers.size());
				// System.out.println(currString);
			}
			currentNumber++;
		}
		System.out.println(df.format(numbers.get(numbers.size() - 1)));
	}

	public static String repeatString(String s, int times) {
		StringBuilder sb = new StringBuilder();
		for (int i = 0; i < times; i++)
		{
			sb.append(s);
		}
		return sb.toString();
	}
}

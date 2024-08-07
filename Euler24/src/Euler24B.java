import java.text.DecimalFormat;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.stream.Collectors;

public class Euler24B {
	public static void main(String[] args) {
		ArrayList<Double> foundNumbers = new ArrayList<>();
		final double GOAL = 5;
		Integer[] NUMBERS = { 0, 1, 2};
		// final double GOAL = 1e6;
		// double currentNumber = 0123456789d;
		// final Integer[] NUMBERS = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9 };
		double currentNumber = arrayToDouble(NUMBERS);
		// This is the set of all the numbers that are present in the number
		// we're currently
		// looking at.
		MyHashMap<Character, Boolean> currSet;
		// This is the string representation of the number we're currently
		// looking at.
		String currString;
		// This is the thing that formats the numbers into strings like we want
		// them.
		final DecimalFormat FORMAT = new DecimalFormat(repeatString("0",
				NUMBERS.length));

		while (foundNumbers.size() < GOAL) {
			currSet = new MyHashMap<>();
			currString = FORMAT.format(currentNumber);
			for (int i = 0; i < currString.length(); i++) {
				currSet.put(currString.charAt(i), Boolean.TRUE);
			}
			if (currSet.findAllInKeys(Arrays.asList(NUMBERS).stream()
					.map(x -> String.valueOf(x).charAt(0))
					.collect(Collectors.toList()))) {
				foundNumbers.add(currentNumber);
				currentNumber = flipLastTwoDigits(currentNumber);
				foundNumbers.add(currentNumber);
				// System.out.println(FORMAT.format(currentNumber));
				if (foundNumbers.size() % 1000 == 0)
					System.out.printf("Found %d%n", foundNumbers.size());
			}
			currentNumber++;
		}
		System.out
				.println(FORMAT.format(foundNumbers.get(foundNumbers.size() - 1)));
	}

	public static String repeatString(String s, int times) {
		StringBuilder sb = new StringBuilder();
		for (int i = 0; i < times; i++) {
			sb.append(s);
		}
		return sb.toString();
	}

	public static double flipLastTwoDigits(double d) {
		char[] c = String.valueOf((int) d).toCharArray();
		char temp = c[c.length - 1];
		c[c.length - 1] = c[c.length - 2];
		c[c.length - 2] = temp;
		return Double.valueOf(new String(c));
	}

	public static double arrayToDouble(Integer[] a) {
		StringBuilder sb = new StringBuilder();
		for (Integer i : a) {
			sb.append(i);
		}
		return Double.valueOf(Integer.valueOf(sb.toString()));
	}
}

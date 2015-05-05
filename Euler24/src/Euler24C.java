import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;
import java.util.stream.Collectors;

public class Euler24C {

	public static void main(String[] args) {
		// final double GOAL = 5;
		// Integer[] NUMBERS = { 0, 1, 2 };
		final double GOAL = 1e6;
		final Integer[] NUMBERS = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9 };
		List<Integer> currentNumber = Arrays.stream(NUMBERS).collect(
				Collectors.toList());
		for (int i = 0; i < GOAL - 1; i++) {
			currentNumber = nextPermutation(currentNumber);
		}
		System.out.println(currentNumber.toString().replace(" ", "")
				.replace(",", "").replace("[", "").replace("]", ""));
	}

	public static List<Integer> nextPermutation(List<Integer> d) {
		int endOfSuffix = d.size() - 1;
		while (endOfSuffix > 0 && d.get(endOfSuffix) <= d.get(endOfSuffix - 1)) {
			--endOfSuffix;
		}
		if (endOfSuffix == 0) {
			System.out.println("It is as big as it gets");
			// Collections.reverse(d);
			return d;
		}
		int pivotPosition = endOfSuffix - 1;
		Integer pivot = d.get(pivotPosition);
		for (int i = d.size() - 1; i > 0; --i) {
			if (d.get(i) > pivot) {
				Collections.swap(d, pivotPosition, i);
				List<Integer> temp = new ArrayList<Integer>(d.subList(
						endOfSuffix, d.size()));
				d.subList(endOfSuffix, d.size()).clear();
				Collections.reverse(temp);
				d.addAll(temp);
				break;
			}
		}
		return d;
	}
}

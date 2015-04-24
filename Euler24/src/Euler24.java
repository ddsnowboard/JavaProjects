import java.util.ArrayList;
import java.util.Arrays;

public class Euler24 {

	public static ArrayList<ArrayList<Integer>> permutations(int[] orig) {
		ArrayList<ArrayList<Integer>> out = new ArrayList<ArrayList<Integer>>();
		ArrayList<Integer> currentPrefix;
		for (Integer i : orig) {
			currentPrefix = new ArrayList<Integer>();
			currentPrefix.add(i);
			out.addAll(permuatationsAux(Arrays.copyOfRange(orig, 1, orig.length),
					currentPrefix));
		}
		return out;
	}

	public static ArrayList<ArrayList<Integer>> permuatationsAux(int[] orig,
			ArrayList<Integer> prefix) {
		ArrayList<ArrayList<Integer>> out = new ArrayList<ArrayList<Integer>>();
		if (orig.length == 1) {
			out.add(orig[0]);
			return out;
		}
		ArrayList<Integer> currPrefix;
		for (Integer i : orig) {
			currPrefix = new ArrayList<Integer>();
			currPrefix.addAll(prefix);
			currPrefix.add(i);
			out.addAll(permuatationsAux(Arrays.copyOfRange(orig, 1, orig.length), currPrefix));
		}
		return out;
	}

	public static void main(String[] args) {
		final int[] NUMBERS = { 1, 2, 3, 4, 5, 6, 7, 8, 9, 0 };
		System.out.println(permutations(NUMBERS));

	}

}

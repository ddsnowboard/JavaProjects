import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;

public class Euler24 {

	public static ArrayList<List> permutations(Integer[] orig) {
		ArrayList<List> out = new ArrayList<List>();
		ArrayList<Integer> currentPrefix;
		ArrayList<List> currentPermutation;
		if(orig.length == 2)
		{
			out.add(Arrays.asList(orig));
			ArrayList<Integer> temp = (ArrayList<Integer>) Arrays.asList(orig);
			Collections.reverse(temp);
			out.add((ArrayList<Integer>) temp);
			return out;
		}
		for (int i = 0; i < orig.length; i++) {
			currentPrefix = new ArrayList<Integer>();
			currentPrefix.add(orig[i]);
			currentPermutation = permutations(Arrays.copyOfRange(orig, 1,
					orig.length));
			for (int j = 0; j < currentPermutation.size(); j++) {
				currentPermutation.get(j).addAll(0, currentPrefix);
				out.add(currentPermutation.get(j));
			}
		}
		return out;
	}

	public static void main(String[] args) {
		// final Integer[] NUMBERS = { 1, 2, 3, 4, 5, 6, 7, 8, 9, 0 };
		final Integer[] NUMBERS = { 1, 2 };
		System.out.println(permutations(NUMBERS));

	}

}

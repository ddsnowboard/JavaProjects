import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;

public class Euler24 {

	public static ArrayList<ArrayList<Integer>> permutations(
			ArrayList<Integer> orig) {
		ArrayList<ArrayList<Integer>> out = new ArrayList<>();
		ArrayList<Integer> workingPrefix;
		ArrayList<ArrayList<Integer>> workingSetOfPermutations;
		if (orig.size() == 2) {
			out.add(new ArrayList<>(orig));
			// There has to be a better way to do this.
			ArrayList<Integer> listToBeReversed = new ArrayList<>(orig);
			Collections.reverse(listToBeReversed);
			out.add(new ArrayList<>(listToBeReversed));
			return out;
		}
		ArrayList<Integer> currList;
		ArrayList<Integer> workingOrig;
		for (int i = 0; i < orig.size(); i++) {
			workingOrig = new ArrayList<>(orig);
			workingPrefix = new ArrayList<Integer>();
			workingPrefix.add(workingOrig.remove(i));
			workingSetOfPermutations = permutations(workingOrig);
			for (int j = 0; j < workingSetOfPermutations.size(); j++) {
				currList = workingSetOfPermutations.get(j);
				for (Integer l : workingPrefix) {
					currList.add(0, l);
				}
				out.add(workingSetOfPermutations.get(j));
			}
		}
		return out;
	}

	public static void main(String[] args) {
		 final Integer[] NUMBERS = { 1, 2, 3, 4, 5, 6, 7, 8, 9, 0 };
//		final Integer[] NUMBERS = { 1, 2, 65, 202, 125, 255, 121, 852, 258,
//				987, 654, 321, 205 };
		System.out.println(permutations(new ArrayList<>(Arrays.asList(NUMBERS))));

	}

}

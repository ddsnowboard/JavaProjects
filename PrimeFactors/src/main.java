import java.util.ArrayList;
import java.util.List;

public class main {

	public static void main(String[] args) {
		final int number = 9;
		int running = number;
		int test = 2;
		ArrayList<Integer> factors = new ArrayList<>();
		while(!(test >= number / 2))
		{
			if((running % test == 0) && !(running <= 2)) {
				factors.add(test);
				running = running / test;
				test = 2;
			} else {
				test++;
			}
		}

		if(!factors.isEmpty()) {
			for(int i = 0; i < factors.size(); i++) {
				System.out.println(factors.get(i));
			}
		} else {
			System.out.printf("%d is prime!", number);
		}

	}

}

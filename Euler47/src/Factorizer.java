import java.util.ArrayList;

public class Factorizer {
	private final static int DEFAULT_NUMBER_OF_PRIMES = 10;
	private final static int STARTING_CURR = -1;
	ArrayList<Integer> primes = new ArrayList<Integer>();

	public Factorizer(int baseNumPrimes) {
		if (baseNumPrimes > 0) {
			fillPrimes(baseNumPrimes);
		} else {
			fillPrimes(DEFAULT_NUMBER_OF_PRIMES);
		}
	}

	public Factorizer() {
		this(DEFAULT_NUMBER_OF_PRIMES);
	}

	private void fillPrimes(double d) {
		int curr = STARTING_CURR;
		boolean prime = true;
		// Short circuit code, ladies and gentlemen.
		while (primes.size() < 1 || primes.get(primes.size() - 1) <= d) {
			if (curr == STARTING_CURR) {
				try {
					curr = primes.get(primes.size() - 1) + 2;
				} catch (ArrayIndexOutOfBoundsException e) {
					primes.add(2);
					primes.add(3);
					curr = 5;
				}
			}
			prime = true;
			for (int i = 3; i < curr / 2 + 1; i++) {
				if (curr % i == 0) {
					prime = false;
					break;
				}
			}
			if (prime) {
				primes.add(curr);
			}
			curr += 2;
		}
	}

	public ArrayList<Integer> primeFactors(int input) {
		ArrayList<Integer> out = new ArrayList<Integer>();
		if (primes.get(primes.size() - 1) < Math.sqrt(input))
			fillPrimes((int) (input / 2) + 1);
		for (Integer i : primes) {
			if (i < input) {
				if (input % i == 0) {
					out.add(i);
				}
			} else {
				break;
			}
		}
		return out;
	}
}

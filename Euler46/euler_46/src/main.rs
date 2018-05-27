use std::collections::HashSet;

type Num = u32;

struct PrimeSet {
    // I think what this should do is have one method: contains. It will calculate all the primes
    // up to (and obviously including) the given number and cache them. If it sees a number bigger
    // than the biggest number it's seen, it calculates more, and if it sees a smaller number, it
    // just checks the set and returns whether there's anything in there.
    Num highest_number;
    HashSet<Num> primes;
}

impl PrimeChecker {
    fn is_prime(&mut self, i: u32) -> bool {
        match primes.entry(i) {
            Occupied(ent) => ent.get(),
            Vacant(ent) => 
        }
    }

    fn check_prime(i: u32) -> bool {
        for divisor in (1..)
    }
}

fn main() {
    println!("Hello, world!");
}

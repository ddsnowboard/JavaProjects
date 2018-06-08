use std::collections::HashSet;
use std::collections::hash_set::Iter;

type Num = u32;

struct PrimeSet {
    // I think what this should do is have one method: contains. It will calculate all the primes
    // up to (and obviously including) the given number and cache them. If it sees a number bigger
    // than the biggest number it's seen, it calculates more, and if it sees a smaller number, it
    // just checks the set and returns whether there's anything in there.
    highest_number: Num,
    primes: HashSet<Num>
}

impl PrimeSet {
    fn new() -> Self {
        const TOP: Num = 7;
        let mut out = PrimeSet { 
            highest_number: TOP,
            primes: HashSet::new()
        };
        out.primes.insert(2);
        out.primes.insert(3);
        out.primes.insert(5);
        out.primes.insert(7);
        out
    }

    fn is_prime(&mut self, i: Num) -> bool {
        if i >= self.highest_number {
            while self.highest_number < i {
                let next_highest = self.highest_number + 2;
                self.fill(next_highest);
            }
        }  
        self.primes.contains(&i)
    }

    fn fill(&mut self, i: Num) {
        type float = f64;
        if (i as float).sqrt() > (self.highest_number as float) {
            panic!("You need to fill in the set first!");
        } else if self.highest_number > i {
            return;
        } else {
            self.highest_number = i;
            for &prime in &self.primes {
                if i % prime == 0 {
                    return;
                }
            }
            self.primes.insert(i);
        }
    }

    fn iter(&self) -> Iter<Num> {
        self.primes.iter()
    }
}

fn goldbach(primes: &mut PrimeSet, i: Num) -> bool {
    let total = |p, s| p + 2 * s;
    let primes = primes.iter().filter(|&&x| x < i);
    let squares: Vec<_> = (1..).map(|x| x * x).take_while(|&x| x < i).collect();
    for p in primes {
        let squares = squares.iter();
        for &s in squares {
            if total(p, s) == i {
                return true;
            }
        }
    }
    false
}

fn main() {
    let mut primes = PrimeSet::new();
    let (first, _) = (1..)
        .filter(|x| x % 2 == 1)
        .filter(|x| !primes.is_prime(*x))
        .map(|x| (x, goldbach(&mut primes, x)))
        .skip_while(|(_, x) | !x)
        .nth(0)
        .unwrap();
    println!("First is {}", first);
}

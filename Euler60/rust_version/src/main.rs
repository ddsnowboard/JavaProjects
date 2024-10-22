use itertools::Itertools;
use lazy_static::lazy_static;
use rayon::iter::ParallelBridge;
use rayon::prelude::*;
use std::collections::HashSet;
use std::fs::read_to_string;

type Num = u16;

lazy_static! {
    // static ref primes: HashSet<Num> = read_to_string("../primes.txt")
     static ref primes: HashSet<Num> = read_to_string("../somePrimes.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse::<Num>().unwrap())
        .collect();
}

fn main() {
    // This doesn't work because itertools and rayon don't play nice. Maybe I can use a crossbeam
    // channel for this. It's just like a go channel. How bad can it be?
    let all_combinations: Vec<_> = primes
        .iter()
        .filter(|p| **p < 1000)
        .combinations(4)
        .chunks(1_000_000)
        .iter()
        .par_bridge()
        .flat_map(|vv| vv.filter(|v|is_prime_pair_set(v)))
        .for_each(|s| println!("{:?}", s));
}

fn is_prime_pair_set(s: &[&Num]) -> bool {
    fn is_concat_prime(l: &Num, r: &Num) -> bool {
        let concat: Num = format!("{}{}", l, r).parse().unwrap();
        primes.contains(&concat)
    }
    s.iter()
        .permutations(2)
        .all(|l| is_concat_prime(l[0], l[1]))
}

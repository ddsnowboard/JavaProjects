#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate itertools;

use rand::seq::SliceRandom;
use rayon::prelude::*;
use std::sync::Arc;

type Strategy = (i32, i32);

lazy_static! {
    static ref STRATEGIES: Vec<Strategy> = iproduct!(0..=100, 0..=100).collect();
    static ref VOTERS: &'static Vec<Strategy> = &STRATEGIES;
}

fn main() {
    simulate_iteration();
}

fn distance(a: Strategy, b: Strategy) -> f64 {
    let (ax, ay) = a;
    let (bx, by) = b;
    (((ax - bx) * (ax - bx) + (ay - by) * (ay - by)) as f64).sqrt()
}

fn voter_is_closer(voter: Strategy, me: Strategy, other_dist: f64) -> bool {
    distance(voter, me) < other_dist
}

fn best_strategy(other: Strategy) -> Strategy {
    let distances: Arc<Vec<_>> = Arc::new(VOTERS.par_iter().map(|v| distance(*v, other)).collect());
    *STRATEGIES
        .par_iter()
        .max_by_key(|s| {
            VOTERS
                .iter()
                .zip(distances.iter())
                .filter(|(v, distance_to_other)| voter_is_closer(**v, **s, **distance_to_other))
                .count()
        })
        .unwrap()
}

fn simulate_iteration() {
    let mut rand = rand::thread_rng();
    let mut start_strategies: Vec<_> = STRATEGIES.choose_multiple(&mut rand, 2).collect();
    start_strategies.shuffle(&mut rand);
    let (mut s1, mut s2) = (*start_strategies[0], *start_strategies[1]);
    loop {
        println!("{:?}", (s1, s2));
        let new_s1 = best_strategy(s2);
        let new_s2 = best_strategy(new_s1);
        if (new_s1, new_s2) == (s1, s2) {
            break;
        }
        s1 = new_s1;
        s2 = new_s2;
    }
}

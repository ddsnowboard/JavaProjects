use crossbeam::channel::bounded;
use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::thread;

type Num = u32;

/**
  This isn't using a N_THREADS threads because it turns out that generating the chunks
  is actually slower than checking if they're prime, so most of the worker threads are sleeping. Not really sure how to
  fix this. I have a suspicion that all the allocations (for example, the line c.collect::<Vec<_>>()) are being slow, but getting rid of those
  is quite challenging, turns out.
 */
const N_THREADS: usize = 5;

lazy_static! {
    static ref primes: HashSet<Num> = read_to_string("../primes.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse::<Num>().unwrap())
        .collect();
}

fn main() {
    let chunk_iterable = primes
        .iter()
        .cloned()
        .filter(|p| *p < 10000)
        .combinations(5)
        .chunks(1_000_000);
    let all_combinations = chunk_iterable.into_iter().map(|c| c.collect::<Vec<_>>());
    let (send_tx, send_rx) = bounded::<Vec<Vec<Num>>>(1000);
    let (recv_tx, recv_rx) = bounded::<Vec<Num>>(1000000);

    let sets = thread::scope(|s| {
        // This just receives stuff and adds it to the list
        let catcher = s.spawn(|| {
            let mut out: Vec<Vec<Num>> = vec![];
            recv_rx.iter().for_each(|s| {
                out.push(s);
            });
            out
        });

        // These are the workers
        {
            // These need to be dropped or else this code will run forever.
            let send_rx = send_rx;
            let recv_tx = recv_tx;
            (0..N_THREADS).for_each(|_| {
                let get_work = send_rx.clone();
                let send_results = recv_tx.clone();
                s.spawn(move || {
                    get_work.iter().for_each(|sets| {
                        for s in sets.into_iter() {
                            if is_prime_pair_set(&s) {
                                send_results.send(s).unwrap();
                            }
                        }
                    })
                });
            });
        }
        all_combinations.for_each(|chunk| {
            send_tx.send(chunk).unwrap();
        });
        drop(send_tx);
        catcher.join().unwrap()
    });
    for s in sets {
        println!("{:?}", s);
    }
}

fn is_prime_pair_set(s: &[Num]) -> bool {
    fn is_concat_prime(l: &Num, r: &Num) -> bool {
        let concat: Num = format!("{}{}", l, r).parse().unwrap();
        primes.contains(&concat)
    }
    s.iter()
        .permutations(2)
        .all(|l| is_concat_prime(l[0], l[1]))
}

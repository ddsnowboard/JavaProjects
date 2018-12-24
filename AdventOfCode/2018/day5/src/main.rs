extern crate num_cpus;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem;
use std::sync::Arc;
use std::thread;

fn main() {
    let input = Arc::new(parse_input());
    let input = Arc::clone(&input);
    let mut distinct_characters: HashSet<char> = HashSet::new();
    for c in input.chars().filter(|c| c.is_uppercase()) {
        distinct_characters.insert(c);
    }
    let mut distinct_characters: Vec<char> = distinct_characters.into_iter().collect();
    let cpus = num_cpus::get();
    let block_size = (distinct_characters.len() as f64 / (cpus as f64)).ceil() as usize;
    let mut even_groups: Vec<Vec<char>> = Vec::new();
    for _ in 0..cpus {
        let split_spot = if block_size > distinct_characters.len() {
            distinct_characters.len()
        } else {
            block_size
        };
        let mut next = distinct_characters.split_off(split_spot);
        mem::swap(&mut next, &mut distinct_characters);
        even_groups.push(next);
    }
    assert!(even_groups.len() == cpus);
    for l in even_groups.iter().map(|l| l.len()) {
        println!("{}", l);
    }
    let handles: Vec<_> = even_groups
        .into_iter()
        .map(|cs| {
            let input = Arc::clone(&input);
            thread::spawn(move || {
                let min = cs
                    .iter()
                    .map(|c| {
                        reduce_all(
                            &input
                                .chars()
                                .filter(|cc| !c.eq_ignore_ascii_case(cc))
                                .collect::<String>(),
                        )
                        .len()
                    })
                    .min();
                min
            })
        })
        .collect();

    if let Some(min) = handles
        .into_iter()
        .map(|t| t.join().unwrap())
        .filter_map(|x| if let Some(min) = x { Some(min) } else { None })
        .min()
    {
        println!("{}", min);
    } else {
        panic!("Well I'm confused");
    }
}

fn parse_input() -> String {
    const INPUT_FILE: &str = "input.txt";
    let mut l = BufReader::new(File::open(INPUT_FILE).unwrap());
    let mut out = String::new();
    l.read_line(&mut out).unwrap();
    out.trim_end().chars().collect()
}

fn has_matching_pair(s: &mut Vec<char>) -> Option<()> {
    for idx in 0..s.len() - 1 {
        let (a, b) = (s[idx], s[idx + 1]);
        if a != b && a.eq_ignore_ascii_case(&b) {
            s.remove(idx);
            s.remove(idx);
            return Some(());
        }
    }
    None
}

fn reduce_all(s: &str) -> String {
    let mut arr: Vec<_> = s.chars().collect();
    while let Some(()) = has_matching_pair(&mut arr) {}
    arr.into_iter().collect()
}

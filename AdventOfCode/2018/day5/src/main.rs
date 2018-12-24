extern crate num_cpus;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;

fn main() {
    let input = parse_input();
    let mut distinct_characters: HashSet<char> = HashSet::new();
    for c in input.chars() {
        distinct_characters.insert(c);
    }
    let mut distinct_characters: Vec<char> = distinct_characters.into_iter().collect();
    let cpus = num_cpus::get();
    let block_size = distinct_characters.len() / cpus;
    let mut even_groups: Vec<Vec<char>> = Vec::new();
    for _ in 0..cpus {
        even_groups.push(distinct_characters.split_off(block_size));
    }
    if let Some(min) = even_groups
        .iter()
        .map(|cs| {
            thread::spawn(|| {
                cs.iter()
                    .map(|c| {
                        reduce_all(
                            &input
                                .chars()
                                .filter(|cc| !c.eq_ignore_ascii_case(cc))
                                .collect::<String>(),
                        )
                        .len()
                    })
                    .min()
            })
        })
        .map(|t| t.join().unwrap())
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

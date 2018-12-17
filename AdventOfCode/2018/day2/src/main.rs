use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead};
/*
fn main() {
    let ids: Vec<String> = read_input();
    let mut twos = 0;
    let mut threes = 0;
    for id in &ids {
        let (two, three) = count_letters(id);
        if two {
            twos += 1;
        }
        if three {
            threes += 1;
        }
    }
    println!("{}", twos * threes);
}
*/
fn main() {
    let ids: Vec<String> = read_input();
    for x in &ids {
        for y in &ids {
            if x == y {
                continue;
            }
            if let Some(diff) = find_diff(x, y) {
                println!("{}", diff);
                return;
            }
        }
    }
}

fn read_input() -> Vec<String> {
    const INPUT_FILE: &str = "input.txt";
    let pth = Path::new(INPUT_FILE);
    let file = File::open(pth).unwrap();
    let lines = BufReader::new(file).lines();
    return lines
        .map(|l| l.unwrap())
        .collect();
}

fn count_letters(s: &str) -> (bool, bool) {
    let mut m = HashMap::new();
    for c in s.chars() {
        let e = m.entry(c).or_insert(0);
        *e += 1;
    }
    (m.values().any(|&i| i == 2), m.values().any(|&i| i == 3))
}

fn find_diff(a: &str, b: &str) -> Option<String> {
    let count_different = a.chars()
        .zip(b.chars())
        .filter(|(aa, bb)| aa != bb)
        .count();
    if count_different == 1 {
        Some(a.chars()
        .zip(b.chars())
        .filter(|(aa, bb)| aa == bb)
        .map(|(aa, _)| aa)
        .collect())
    } else {
        None
    }
}

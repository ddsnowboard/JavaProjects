use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut input: Vec<char> = parse_input();
    while let Some(new) = has_matching_pair(&input) {
        input = new;
    }
    println!("{}", input.len());
}

fn parse_input() -> Vec<char> {
    const INPUT_FILE: &str = "input.txt";
    let mut l = BufReader::new(File::open(INPUT_FILE).unwrap());
    let mut out = String::new();
    l.read_line(&mut out).unwrap();
    out.chars().collect()
}

fn has_matching_pair(s: &str) -> Option<String> {
    for idx in 0..s.len() - 1 {
        let (a, b) = (s[idx], s[idx + 1]);
    }
}

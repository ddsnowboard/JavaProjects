use std::collections::HashSet;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {
    let input = get_input();
    let words = get_wordset();
}

fn get_input() -> String { 
    use std::env::args;
    use std::process::exit;
    let mut arguments = args();
    if let Some(s) = arguments.nth(1) {
        s
    } else {
        println!("USAGE: {} LETTERS", arguments.nth(0).unwrap());
        exit(1);
    }
}

fn get_wordset() -> HashSet<String> {
    const WORDS_FILE: &'static str = "/usr/share/dict/usa";
    let mut reader = BufReader::new(File::open(WORDS_FILE).unwrap());
    let mut out = HashSet::new();

    for word in reader.lines() {
        if let Ok(s) = word {
            out.insert(s);
        } else if let Err(e) = word {
            println!("I don't understand the problem");
            panic!(e);
        }
    }
    out
}

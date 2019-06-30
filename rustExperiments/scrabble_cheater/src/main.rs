use std::collections::HashSet;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::Iterator;
use std::collections::hash_map::Iter;

fn main() {
    use std::process::exit;
    use std::env::args;
    if let Some(input) = get_input() {
        let dict = get_wordset();
        let chars = Counter::from_iter(&mut input.chars());
        let with_letter_counts = dict.iter().map(|word| (word, Counter::from_iter(&mut word.chars())));
        let good_words = with_letter_counts.filter(|(_, ctr)| chars.is_superset(ctr))
            .map(|(word, _)| word);
        for word in good_words {
            println!("{}", word);
        }
    } else {
        let arguments: Vec<_> = args().collect();
        println!("USAGE: {} LETTERS", arguments[0]);
        exit(1);
    }
}

fn get_input() -> Option<String> { 
    use std::env::args;
    let arguments: Vec<_> = args().collect();
    if arguments.len() == 2 {
        Some(arguments[1].clone())
    } else {
        None
    }
}

fn get_wordset() -> HashSet<String> {
    const WORDS_FILE: &'static str = "/usr/share/dict/words";
    let reader = BufReader::new(File::open(WORDS_FILE).unwrap());
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

struct Counter<T : Hash + Eq> {
    dict: HashMap<T, u32>
}

impl<T : Hash + Eq> Counter<T> {
    fn new() -> Self {
        Counter { dict: HashMap::new() }
    }

    fn from_iter(it: &mut Iterator<Item = T>) -> Self {
        let mut out = Self::new();
        while let Some(el) = it.next() {
            out.put(el);
        }
        out
    }

    fn put(&mut self, el: T) {
        let ptr = self.dict.entry(el).or_insert(0);
        *ptr += 1;
    }

    fn get(&self, el: &T) -> u32 {
        match self.dict.get(el) {
            Some(x) => *x,
            None => 0
        }
    }

    fn iter(&self) -> Iter<T, u32> {
        self.dict.iter()
    }

    fn is_superset(&self, other: &Self) -> bool {
        other.dict.iter().all( |(key, &count)| self.get(key) >= count)
    }

    fn contains(&self, el: &T) -> bool {
        self.get(el) > 0
    }

    fn is_subset(&self, other: &Self) -> bool {
        !other.is_superset(&self)
    }
}

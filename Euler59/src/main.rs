extern crate rayon;

use rayon::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::char;
use std::u32;
use std::convert::From;
use std::io::BufReader;

const KEY_LENGTH: usize = 3;
const LONGEST_WORDS_IN_DICTIONARY: usize = 26;
fn main() {
    let mut contents = String::new();
    const FILENAME: &'static str = "cipher.txt";
    {
        let mut file = File::open(FILENAME).unwrap();
        file.read_to_string(&mut contents).unwrap();
    }
    let ascii_values = to_vec_of_ints(&contents);

    let dict = Dictionary::new();

    let keys: Vec<Vec<char>> = KeyIterator::new(KEY_LENGTH).collect();

    let max_pair = keys.par_iter().map(| key | {
        let decrypted = decrypt(&ascii_values, &key);
        let words: Vec<&str> = decrypted.split_whitespace().collect();
        if words.iter().all(|x| x.len() <= LONGEST_WORDS_IN_DICTIONARY) {
            let count = dict.union(&words);
            Some((key.clone(), count))
        }
        else {
            None
        }
    }).max_by_key(| &pair | {
        match pair {
            Some((_, c)) => c,
            None => 0,
        }
    });

    // The outer Option is from max_by_key(), the inner one is from the lambda I gave map()
    if let Some(Some((biggest_key, biggest_count))) = max_pair
    {
        // Does this need iter or into_iter? Why do I need cloned()?
        let key_as_string: String = biggest_key.cloned().iter().collect();
        println!("Most was {} with {}", biggest_count, key_as_string);
        let real_message = decrypt(&ascii_values, &biggest_key);
        println!("Message is \n{}", real_message);
        println!("Sum is {}", real_message.as_bytes().iter().map(|x| *x as u32).sum::<u32>());
    }
    else {
        println!("We didn't find anything");
    }
}

fn to_vec_of_ints(string: &str) -> Vec<u32> {
    string.split(",").map(|x| x.trim())
        .map(|x| x.parse().unwrap())
        .collect()
}

fn decrypt(characters: &[u32], key: &[char]) -> String {
    let cycler = key.iter().map(|x| u32::from(*x)).cycle();
    characters.iter().zip(cycler)
        .map(|(c, k)| *c ^ k)
        .map(|x| std::char::from_u32(x).unwrap())
        .collect()
}

struct KeyIterator {
    current_password: Vec<char>, 
    password_length: usize,
}

impl KeyIterator {
    fn new(i: usize) -> KeyIterator {
        KeyIterator {current_password: vec!['a'; i], password_length: i}
    }
}

impl Iterator for KeyIterator {
    type Item = Vec<char>;

    // Why doesn't this return a slice or a borrow?
    fn next(&mut self) -> Option<Vec<char>> {
        for idx in (0..self.password_length).rev() {
            if self.current_password[idx] == 'z' {
                self.current_password[idx] = 'a';
                continue;
            }
            else {
                self.current_password[idx] = std::char::from_u32(u32::from(self.current_password[idx]) + 1).unwrap();
                return Some(self.current_password.clone());
            }
        }
        None
    }
}

struct Dictionary {
    words: HashSet<String>;
}

impl Dictionary {
    fn new() -> Dictionary {
        const FILE_PATH: &'static str = "/usr/share/dict/words";
        let file = File::open(FILE_PATH).unwrap();
        let reader = BufReader::new(&file);
        Dictionary { words: reader.lines().map(|x| String::from(x.unwrap()) ).collect() } 
    }

    fn contains(&self, word: &str) -> bool {
        let word = word.trim().to_lowercase();
        self.words.contains(word)
    }

    fn union(&self, other_words: &[&str]) -> u32 {
        const WORDS_TO_CHECK: usize = 20;
        other_word.iter().take(WORDS_TO_CHECK)
            .map(|word| match self.contains(word) { true => 1, false => 0 })
            .sum()
    }
}


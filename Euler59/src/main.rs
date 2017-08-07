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

    let mut max = 0;
    let mut max_key: Vec<char> = vec![];
    for key in KeyIterator::new(KEY_LENGTH) {
        let decrypted = decrypt(&ascii_values, &key);
        let words: Vec<&str> = decrypted.split_whitespace().collect();
        if (&words).par_iter().all(|x| x.len() <= LONGEST_WORDS_IN_DICTIONARY) {
            let count = dict.union(&words);
            if count > max {
                max = count;
                max_key = key.clone();
            }
        }
    };
    let key_as_string: String = max_key.iter().cloned().collect();
    println!("Most was {} with {}", max, key_as_string);
    let real_message = decrypt(&ascii_values, &max_key);
    println!("Message is \n{}", real_message);
    println!("Sum is {}", real_message.as_bytes().into_iter().map(|x| *x as u32).sum::<u32>());
}

fn to_vec_of_ints(string: &str) -> Vec<u32> {
    string.split(",").map(|x| x.trim())
        .map(|x| x.parse().unwrap())
        .collect()
}

fn decrypt(characters: &[u32], key: &[char]) -> String {
    let cycler = key.into_iter().map(|x| u32::from(*x)).collect::<Vec<u32>>().into_iter().cycle();
    characters.into_iter().zip(cycler)
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
    words: Vec<String>
}

impl Dictionary {
    fn new() -> Dictionary {
        const FILE_PATH: &'static str = "/usr/share/dict/words";
        let file = File::open(FILE_PATH).unwrap();
        let reader = BufReader::new(&file);
        Dictionary { words: reader.lines().map(|x| x.unwrap()).collect() } 
    }

    fn union(&self, other_words: &[&str]) -> u32 {
        const WORDS_TO_CHECK: usize = 20;
        let mut counter = 0;
        for other_words in other_words.into_iter().take(WORDS_TO_CHECK) {
            let word = other_words.trim().to_lowercase();
            for line in &self.words {
                let line = String::from(line.trim());
                if line == word {
                    counter += 1;
                    break;
                }
            }
        }
        counter
    }
}


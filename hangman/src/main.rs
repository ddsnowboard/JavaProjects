use hangman;

use rayon::prelude::*;
use std::io::{stdin, BufRead, BufReader};

const COUNT: usize = 10000;

fn main() {
    /*
    println!(
        "{}",
        hangman::run_game(read_input(), &mut hangman::DictionaryStrategy::default())
    )
    */

    let words: Vec<String> = hangman::DICTIONARY.iter().take(COUNT).cloned().collect();
    let wins = words
        .into_par_iter()
        .take(COUNT)
        .map(|w| hangman::run_game(w, &mut hangman::DictionaryStrategy::default()))
        .filter(hangman::GameState::whole_word_guessed)
        .count() as f32;
    println!("{}%", wins * 100.0 / COUNT as f32);
}

fn read_input() -> String {
    let reader = BufReader::new(stdin());
    reader.lines().next().unwrap().unwrap()
}

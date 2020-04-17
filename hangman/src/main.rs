use hangman;
use rayon::prelude::*;

use std::io::{stdin, BufRead, BufReader};

fn main() {
    /*
    println!(
        "{}",
        hangman::run_game(read_input(), &mut hangman::DictionaryStrategy::default())
    )
    */

    let wins = hangman::DICTIONARY
        .par_iter()
        .map(|w| hangman::run_game(w.to_string(), &mut hangman::DictionaryStrategy::default()))
        .filter(hangman::GameState::whole_word_guessed)
        .count() as f32;
    println!("{}%", wins / hangman::DICTIONARY.len() as f32);
}

fn read_input() -> String {
    let reader = BufReader::new(stdin());
    reader.lines().next().unwrap().unwrap()
}

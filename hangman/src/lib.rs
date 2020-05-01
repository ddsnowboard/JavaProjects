use counter::Counter;
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

const MAX_MISSES: usize = 8;

lazy_static! {
    pub static ref LETTERS: HashSet<char> = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
    ]
    .iter()
    .copied()
    .collect();
    pub static ref DICTIONARY: HashSet<String> = {
        let dict = File::open("enable1.txt").unwrap();
        let dict_reader = BufReader::new(dict);
        dict_reader.lines().map(|c| c.unwrap()).collect()
    };
    pub static ref MOST_COMMON_LETTERS: Vec<char> = {
        let all_letters = DICTIONARY
            .iter()
            .map(|c| c.chars().collect::<Vec<char>>())
            .flatten();
        let letter_map: Counter<char> = all_letters.collect();
        letter_map
            .most_common()
            .into_iter()
            .map(|(a, _)| a)
            .collect()
    };
}

pub trait HangmanStrategy {
    fn guess(&mut self, problem: &ProblemState) -> char;
}

pub struct GameState {
    pub word: String,
    pub guessed: HashSet<char>,
}

impl GameState {
    pub fn new(word: String) -> GameState {
        GameState {
            word,
            guessed: HashSet::new(),
        }
    }

    pub fn current_problem_state(&self) -> ProblemState {
        let slots: Vec<Option<char>> = self
            .word
            .chars()
            .map(|c| {
                if self.guessed.contains(&c) {
                    Some(c)
                } else {
                    None
                }
            })
            .collect();

        let wrong_letters = &self.guessed - &self.word.chars().collect();
        ProblemState {
            slots,
            wrong_letters,
        }
    }

    pub fn guess_letter(&mut self, letter: char) {
        self.guessed.insert(letter);
    }

    pub fn whole_word_guessed(&self) -> bool {
        self.guessed.is_superset(&(self.word.chars().collect()))
    }

    pub fn is_hanged(&self) -> bool {
        let wrong_letters = &self.guessed - &(self.word.chars().collect());
        wrong_letters.len() > MAX_MISSES
    }

    pub fn is_done(&self) -> bool {
        self.whole_word_guessed() || self.is_hanged()
    }

    pub fn letter_been_guessed(&self, letter: char) -> bool {
        self.guessed.contains(&letter)
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fate = if self.whole_word_guessed() {
            "survived"
        } else {
            "died"
        };
        let mut sorted_guesses: Vec<_> = self.guessed.iter().collect();
        sorted_guesses.sort_unstable();
        write!(
            f,
            "Guessed {:?}\nWord was {}\nI {}",
            sorted_guesses, self.word, fate
        )
    }
}

pub struct ProblemState {
    slots: Vec<Option<char>>,
    wrong_letters: HashSet<char>,
}

impl ProblemState {
    fn all_guessed_letters(&self) -> HashSet<char> {
        &self
            .slots
            .iter()
            .filter(|w| w.is_some())
            .map(|w| w.unwrap())
            .collect()
            | &self.wrong_letters
    }
}

#[derive(Default)]
pub struct DumbStrategy {}

#[derive(Default)]
pub struct MediumStrategy {}

impl HangmanStrategy for MediumStrategy {
    fn guess(&mut self, problem: &ProblemState) -> char {
        let all_guesses = &problem
            .slots
            .iter()
            .filter(|w| w.is_some())
            .map(|w| w.unwrap())
            .collect()
            | &problem.wrong_letters;

        if let Some(out) = MOST_COMMON_LETTERS
            .iter()
            .find(|c| !all_guesses.contains(c))
        {
            *out
        } else {
            panic!("I ran out of ideas");
        }
    }
}

impl HangmanStrategy for DumbStrategy {
    fn guess(&mut self, problem: &ProblemState) -> char {
        let available_letters: Vec<_> = (&*LETTERS - &problem.all_guessed_letters())
            .into_iter()
            .collect();
        available_letters[thread_rng().gen_range(0, available_letters.len())]
    }
}

#[derive(Default)]
pub struct DictionaryStrategy {
    current_word_set: Option<HashSet<String>>,
}

impl DictionaryStrategy {
    fn word_matches_slots(word: &str, slots: &[Option<char>]) -> bool {
        debug_assert!(word.len() == slots.len());
        word.chars().zip(slots.iter()).all(|(c, s)| match *s {
            Some(cc) => c == cc,
            None => true,
        })
    }
}

impl HangmanStrategy for DictionaryStrategy {
    fn guess(&mut self, problem: &ProblemState) -> char {
        let matching_words: &HashSet<String> = if let Some(ref mut w) = self.current_word_set {
            w.retain(|word| {
                word.len() == problem.slots.len() && Self::word_matches_slots(word, &problem.slots)
            });
            w
        } else {
            self.current_word_set.get_or_insert(
                DICTIONARY
                    .iter()
                    .filter(|word| word.len() == problem.slots.len())
                    .filter(|w| Self::word_matches_slots(w, &problem.slots))
                    .cloned()
                    .collect(),
            )
        };

        let letters_count: Counter<_> = matching_words.iter().flat_map(|w| w.chars()).collect();
        if let Some((c, _)) = letters_count
            .most_common()
            .into_iter()
            .find(|(c, _)| !problem.all_guessed_letters().contains(c))
        {
            c
        } else {
            let used_characters = problem.all_guessed_letters();
            *MOST_COMMON_LETTERS
                .iter()
                .find(|c| !used_characters.contains(c))
                .unwrap()
        }
    }
}

pub fn run_game<T: HangmanStrategy>(word: String, strategy: &mut T) -> GameState {
    let mut state = GameState::new(word);
    while !state.is_done() {
        let next_guess = strategy.guess(&state.current_problem_state());
        if state.letter_been_guessed(next_guess) {
            panic!("You guessed {} twice", next_guess);
        }
        state.guess_letter(next_guess);
    }
    state
}

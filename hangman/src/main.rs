use counter::Counter;
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

const MAX_MISSES: usize = 8;

lazy_static! {
    static ref LETTERS: HashSet<char> = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
    ]
    .iter()
    .copied()
    .collect();
}

trait HangmanStrategy {
    fn guess(&mut self, problem: &ProblemState) -> char;
}

struct GameState {
    word: String,
    guessed: HashSet<char>,
}

impl GameState {
    fn new(word: String) -> GameState {
        GameState {
            word,
            guessed: HashSet::new(),
        }
    }

    fn current_problem_state(&self) -> ProblemState {
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

    fn guess_letter(&mut self, letter: char) {
        self.guessed.insert(letter);
    }

    fn whole_word_guessed(&self) -> bool {
        self.guessed.is_superset(&(self.word.chars().collect()))
    }

    fn is_hanged(&self) -> bool {
        let wrong_letters = &self.guessed - &(self.word.chars().collect());
        wrong_letters.len() > MAX_MISSES
    }

    fn is_done(&self) -> bool {
        self.whole_word_guessed() || self.is_hanged()
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

struct ProblemState {
    slots: Vec<Option<char>>,
    wrong_letters: HashSet<char>,
}

struct DumbStrategy {}

impl DumbStrategy {
    fn new() -> DumbStrategy {
        DumbStrategy {}
    }
}

struct MediumStrategy {
    priors: Counter<char>,
}

impl HangmanStrategy for MediumStrategy {
    fn guess(&mut self, problem: &ProblemState) -> char {
        let all_guesses = &problem
            .slots
            .iter()
            .filter(|w| w.is_some())
            .map(|w| w.unwrap())
            .collect()
            | &problem.wrong_letters;

        if let Some((out, _)) = self
            .priors
            .most_common()
            .iter()
            .filter(|(c, _)| !all_guesses.contains(c))
            .nth(0)
        {
            *out
        } else {
            panic!("I ran out of ideas");
        }
    }
}

impl MediumStrategy {
    fn new() -> MediumStrategy {
        let dict = File::open("enable1.txt").unwrap();
        let dict_reader = BufReader::new(dict);
        let all_letters = dict_reader
            .lines()
            .map(|w| w.unwrap())
            .map(|c| c.chars().collect::<Vec<char>>())
            .flatten();
        let letter_map: Counter<char> = all_letters.collect();
        MediumStrategy { priors: letter_map }
    }
}

impl HangmanStrategy for DumbStrategy {
    fn guess(&mut self, problem: &ProblemState) -> char {
        let correct_letters = problem
            .slots
            .iter()
            .filter(|w| w.is_some())
            .map(|w| w.unwrap())
            .collect();
        let available_letters: Vec<_> = (&*LETTERS - &(&problem.wrong_letters | &correct_letters))
            .into_iter()
            .collect();
        available_letters[thread_rng().gen_range(0, available_letters.len())]
    }
}

fn main() {
    let mut strategy = MediumStrategy::new();
    let outcome = run_game(read_input().to_lowercase(), &mut strategy);
    println!("{}", outcome)
}

fn read_input() -> String {
    let reader = BufReader::new(stdin());
    reader.lines().nth(0).unwrap().unwrap()
}

fn run_game(word: String, strategy: &mut dyn HangmanStrategy) -> GameState {
    let mut state = GameState::new(word);
    while !state.is_done() {
        let next_guess = strategy.guess(&state.current_problem_state());
        state.guess_letter(next_guess);
    }
    state
}

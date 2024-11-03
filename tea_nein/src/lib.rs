use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::sync::LazyLock;

pub type Number = u8;
static NUMBER_MAPPING: LazyLock<HashMap<Number, Vec<char>>> = LazyLock::new(|| {
    [
        (2, vec!['a', 'b', 'c']),
        (3, vec!['d', 'e', 'f']),
        (4, vec!['g', 'h', 'i']),
        (5, vec!['j', 'k', 'l']),
        (6, vec!['m', 'n', 'o']),
        (7, vec!['p', 'q', 'r', 's']),
        (8, vec!['t', 'u', 'v']),
        (9, vec!['w', 'x', 'y', 'z']),
    ]
    .into()
});

static WORDS: LazyLock<HashSet<String>> = LazyLock::new(|| {
    let file = File::open("data/words").unwrap();
    BufReader::new(file)
        .lines()
        .map(|s| s.unwrap().trim().to_owned())
        .collect()
});

pub fn string_to_numbers(s: &str) -> Vec<Number> {
    s.chars()
        .map(|c| NUMBER_MAPPING.iter().find(|(_, v)| v.contains(&c)).unwrap())
        .map(|(k, _)| k)
        .copied()
        .collect()
}

pub trait Predictor {
    fn pick(&self, options: &[String]) -> String;
    fn predict(&self, number_inputs: &[Number]) -> String {
        let choices: Vec<_> = number_inputs
            .iter()
            .map(|n| NUMBER_MAPPING[n].clone())
            .multi_cartesian_product()
            .map(|charvec| charvec.into_iter().collect::<String>())
            .collect();
        self.pick(&choices)
    }
}

pub struct RandomPredictor {
    /// Picks the first dictionary word that matches and returns it
    dictionary: HashSet<String>,
}

impl RandomPredictor {
    pub fn new() -> Self {
        Self {
            dictionary: WORDS.clone(),
        }
    }
}

impl Default for RandomPredictor {
    fn default() -> Self {
        Self::new()
    }
}

impl Predictor for RandomPredictor {
    fn pick(&self, options: &[String]) -> String {
        options
            .iter()
            .find(|s| self.dictionary.contains(*s))
            .unwrap()
            .to_string()
    }
}

enum MarkovState {
    Letter(char),
    Stop,
}
pub struct HMM {
    log_letter_edge_weights: HashMap<(MarkovState, MarkovState), HashMap<MarkovState, f64>>,
}

impl HMM {
    fn new() -> Self {
I guess I count up the number of times each edge appears and then divide each by the total per (MarkovState, MarkovState)? Not sure
    }
}

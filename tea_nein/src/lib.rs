use counter::Counter;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::sync::LazyLock;

pub type Number = u8;
type Index = i16;
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

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum MarkovState {
    Letter(char),
    Stop,
}
pub struct HMM {
    // I'm going to use 2-grams
    // I'd love to make this generic but I can't figure out how
    log_letter_edge_weights: HashMap<(MarkovState, MarkovState), HashMap<MarkovState, f64>>,
}

impl HMM {
    pub fn new() -> Self {
        let all_transitions = WORDS.iter().flat_map(|s| word_to_transitions(s));
        let grouped_by_first_2_chars = all_transitions.into_group_map_by(|(a, b, _)| (*a, *b));
        let counts: HashMap<(MarkovState, MarkovState), Counter<MarkovState>> =
            grouped_by_first_2_chars
                .into_iter()
                .map(|(first_two, all_transitions)| {
                    (
                        first_two,
                        all_transitions
                            .into_iter()
                            .map(|(_, _, c)| c)
                            .collect::<Counter<_>>(),
                    )
                })
                .collect();

        fn counter_to_log_proportions<T: std::hash::Hash + Eq>(c: Counter<T>) -> HashMap<T, f64> {
            let total_observations: f64 = c.values().sum::<usize>() as f64;
            c.into_iter()
                .map(|(k, count)| (k, (count as f64 / total_observations).log(10.0)))
                .collect()
        }
        Self {
            log_letter_edge_weights: counts
                .into_iter()
                .map(|(k, c)| (k, counter_to_log_proportions(c)))
                .collect(),
        }
    }
}

impl Default for HMM {
    fn default() -> Self {
        Self::new()
    }
}

impl Predictor for HMM {
    // Gotta do this next time
    fn pick(&self, options: &[String]) -> String {
        options
            .iter()
            .map(|s| {
                (
                    s,
                    word_to_transitions(s)
                        .into_iter()
                        .map(|(s1, s2, n)| {
                            self.log_letter_edge_weights
                                .get(&(s1, s2))
                                .and_then(|m| m.get(&n))
                                .unwrap_or(&-1000.0)
                        })
                        .sum::<f64>(),
                )
            })
            // .inspect(|p| println!("{:?}", p))
            .max_by(|(_, l), (_, r)| l.partial_cmp(r).unwrap())
            .unwrap()
            .0
            .to_owned()
    }
}

fn word_to_transitions(word: &str) -> Vec<(MarkovState, MarkovState, MarkovState)> {
    fn try_get(b: &[u8], idx: Index) -> MarkovState {
        let as_option = || {
            let idx: usize = idx.try_into().ok()?;
            Some(char::from_u32(*b.get(idx)? as u32).unwrap())
        };
        match as_option() {
            Some(c) => MarkovState::Letter(c),
            None => MarkovState::Stop,
        }
    }
    let bytes = word.as_bytes();
    (0..word.len())
        .map(|last_index| {
            let last_index = last_index as Index;
            (
                try_get(bytes, last_index - 2),
                try_get(bytes, last_index - 1),
                try_get(bytes, last_index),
            )
        })
        .collect()
}

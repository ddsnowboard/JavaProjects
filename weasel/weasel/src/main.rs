use rayon::prelude::*;

const MAX_OFFSPRING: usize = 1000;
const MIN_OFFSPRING: usize = 1;

fn main() {
    let sample_size = 500;
    let data: Vec<_> = (5..50)
        .into_par_iter()
        .map(|length| weasel_sample(length, sample_size))
        .collect();
    println!("Length,average");
    for d in data.into_iter() {
        let average = { d.observations.iter().sum::<usize>() as f64 / d.observations.len() as f64 };
        println!("{},{}", d.length, average);
    }
}

struct SampleData {
    length: usize,
    observations: Vec<usize>,
}

fn weasel_sample(length: usize, n_samples: usize) -> SampleData {
    let observations = (0..n_samples).map(|_| weasel_test(length)).collect();
    SampleData {
        observations,
        length,
    }
}

fn random_letter() -> char {
    const LETTERS: [char; 27] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', ' ',
    ];
    LETTERS[rand::random_range(..LETTERS.len())]
}
fn random_string(length: usize) -> String {
    (0..length).map(|_| random_letter()).collect()
}

fn offspring(s: &str) -> String {
    const CHANCE_OF_MUTATION: f64 = 0.05;
    s.chars()
        .map(|c| {
            if rand::random_bool(CHANCE_OF_MUTATION) {
                random_letter()
            } else {
                c
            }
        })
        .collect()
}

fn distance(s1: &str, s2: &str) -> usize {
    s1.chars()
        .zip(s2.chars())
        .map(|(c1, c2)| if c1 != c2 { 1usize } else { 0usize })
        .sum()
}

fn weasel_test(length: usize) -> usize {
    let expected_output: String = (0..length).map(|_| ' ').collect();
    let mut curr = random_string(length);
    let mut generations: usize = 0;
    while curr != expected_output {
        let n_offspring = rand::random_range(MIN_OFFSPRING..MAX_OFFSPRING);
        let offspring = (0..n_offspring).map(|_| offspring(&curr));
        curr = offspring
            .min_by_key(|child| distance(&expected_output, child))
            .unwrap();
        generations += 1;
    }
    generations
}

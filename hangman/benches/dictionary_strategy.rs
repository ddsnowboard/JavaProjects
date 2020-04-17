use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hangman;

fn dictionary_benchmark(c: &mut Criterion) {
    let word: String = String::from("anathema");
    c.bench_function("1 word: anathema with dictionary", |b| {
        b.iter(|| {
            benchmark(
                &mut hangman::DictionaryStrategy::default(),
                black_box(&word),
            )
        })
    });
}

fn benchmark<T: hangman::HangmanStrategy>(strategy: &mut T, word: &str) {
    let _ = black_box(hangman::run_game(word.to_string(), strategy));
}

fn random_benchmark(c: &mut Criterion) {
    let word: String = String::from("anathema");
    c.bench_function("1 word: anathema with random", |b| {
        b.iter(|| benchmark(&mut hangman::DumbStrategy::default(), black_box(&word)))
    });
}

criterion_group!(benches, dictionary_benchmark, random_benchmark);
criterion_main!(benches);

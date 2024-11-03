use criterion::{criterion_group, criterion_main, Criterion};
use tea_nein::*;

fn benchmark_prediction(p: &impl Predictor, word: &str, numbers: &[Number]) {
    let result = p.predict(numbers);
}

fn criterion_benchmark(c: &mut Criterion) {
    let test_words = ["apple", "snap", "abdicates", "beetle"];
    test_words.into_iter().for_each(|w| {
        let numbers = string_to_numbers(&w);
        c.bench_function(&format!("random predictor for {}", w), |b| {
            b.iter(|| benchmark_prediction(&RandomPredictor::new(), w, &numbers))
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

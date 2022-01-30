use app::run_generations;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("50 generations", |b| b.iter(|| run_generations(50, false)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

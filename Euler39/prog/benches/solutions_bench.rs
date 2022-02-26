use criterion::{black_box, criterion_group, criterion_main, Criterion};
use prog::solutions;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("solutions for 840", |b| {
        b.iter(|| solutions(black_box(&840)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

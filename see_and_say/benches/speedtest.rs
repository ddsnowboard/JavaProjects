use criterion::{Criterion, black_box, criterion_group, criterion_main};
use see_and_say::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("ll iterate 100 times", |b| {
        b.iter(|| ll_iterator().take(black_box(20)).map(|s| s.len()).sum::<usize>())
    });

    c.bench_function("array iterate 100 times", |b| {
        b.iter(|| array_iterator().take(black_box(20)).map(|s| s.len()).sum::<usize>())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

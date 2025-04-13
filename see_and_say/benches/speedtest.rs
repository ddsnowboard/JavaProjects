use criterion::{Criterion, black_box, criterion_group, criterion_main};
use see_and_say::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    for n in [60usize, 50usize, 40usize, 30usize, 20usize] {
        c.bench_function(&format!("ll iterate {} times", n), |b| {
            b.iter(|| {
                ll_iterator()
                    .take(black_box(n))
                    .map(|s| s.len())
                    .sum::<usize>()
            })
        });

        c.bench_function(&format!("array iterate {} times", n), |b| {
            b.iter(|| {
                array_iterator()
                    .take(black_box(n))
                    .map(|s| s.len())
                    .sum::<usize>()
            })
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

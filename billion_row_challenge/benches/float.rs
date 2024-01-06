use criterion::{black_box, criterion_group, criterion_main, Criterion};
use billion_row_challenge::dumb_parse_number;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse 11.24", |b| b.iter(|| dumb_parse_number(black_box("11.24"))));
    //c.bench_function("parse 11.24", |b| b.iter(|| black_box("11.24").parse::<f64>().unwrap()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

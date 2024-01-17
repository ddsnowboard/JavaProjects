use billion_row_challenge::dumb_parse_number;
use billion_row_challenge::read_row;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn float_bench(c: &mut Criterion) {
    c.bench_function("parse 11.24", |b| {
        b.iter(|| dumb_parse_number(black_box("11.24")))
    });
}

pub fn read_row_bench(c: &mut Criterion) {
    c.bench_function("read row", |b| {
        b.iter(|| read_row(black_box("Luxembourg City;16.9")))
    });
}

criterion_group!(benches, float_bench, read_row_bench);
criterion_main!(benches);

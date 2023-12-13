use criterion::{criterion_group, criterion_main, Criterion};

use aoc::d12::possible_arrangements;

pub fn example_5_repeats(c: &mut Criterion) {
    let input = "?????????????#?#?.? 1,1,4,1,1,1";
    c.bench_function("find 5", |b| b.iter(|| possible_arrangements(input, 5)));
}

criterion_group!(benches, example_5_repeats);
criterion_main!(benches);

use std::num::NonZero;

use criterion::{criterion_group, criterion_main, Criterion};

fn day06_part2_benchmark(c: &mut Criterion) {
    let input = aoc_helper::downloader::get_input(2024, NonZero::new(6).unwrap()).unwrap();
    c.bench_function("day06:2", |b| b.iter(|| aoc::day06::solve_part2(&input)));
}

criterion_group!(benches, day06_part2_benchmark);

criterion_main!(benches);

use std::num::NonZero;

use criterion::{criterion_group, criterion_main, Criterion};

fn day09_part2_benchmark(c: &mut Criterion) {
    let input = aoc_helper::downloader::get_input(2024, NonZero::new(9).unwrap()).unwrap();
    c.bench_function("day09:2", |b| b.iter(|| aoc::day09::solve_part2(&input)));
}

criterion_group!(benches, day09_part2_benchmark);

criterion_main!(benches);

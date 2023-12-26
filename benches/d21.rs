use aoc::common::grid::Grid;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn steps_repeating(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/d21.txt").unwrap();
    let grid: Grid<aoc::d21::Cell> = input.parse().unwrap();
    c.bench_function("steps_repeating 500", |b| {
        b.iter(|| aoc::d21::best_method(&grid, 500))
    });
    c.bench_function("steps_repeating 1000", |b| {
        b.iter(|| aoc::d21::best_method(&grid, 1000))
    });
}

criterion_group!(benches, steps_repeating);
criterion_main!(benches);

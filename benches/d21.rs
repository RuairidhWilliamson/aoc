use aoc::common::grid::Grid;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

pub fn steps_repeating(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/d21.txt").unwrap();
    let grid: Grid<aoc::d21::Cell> = input.parse().unwrap();
    let mut group = c.benchmark_group("steps_repeating");
    for n in (500..=2000).step_by(100) {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter(|| aoc::d21::best_method2(&grid, n))
        });
    }
    group.finish();
}

criterion_group!(benches, steps_repeating);
criterion_main!(benches);

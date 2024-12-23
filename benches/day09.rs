use std::num::NonZero;

use criterion::{criterion_group, criterion_main, Criterion};

fn day09_benchmark(c: &mut Criterion) {
    let input = aoc_helper::downloader::get_input(2024, NonZero::new(9).unwrap()).unwrap();
    let mut group = c.benchmark_group("day09:1");
    group.bench_function("disk1", |b| {
        b.iter(|| {
            let mut disk: aoc::day09::Disk = input.trim().parse().unwrap();
            disk.compact1();
            disk.checksum()
        })
    });
    group.bench_function("disk2", |b| {
        b.iter(|| {
            let mut disk: aoc::day09::Disk2 = input.trim().parse().unwrap();
            disk.compact1();
            disk.checksum()
        })
    });
    group.finish();
    let mut group = c.benchmark_group("day09:2");
    group.bench_function("disk1", |b| {
        b.iter(|| {
            let mut disk: aoc::day09::Disk = input.trim().parse().unwrap();
            disk.compact2();
            disk.checksum()
        })
    });
    group.bench_function("disk2", |b| {
        b.iter(|| {
            let mut disk: aoc::day09::Disk2 = input.trim().parse().unwrap();
            disk.compact2();
            disk.checksum()
        })
    });
    group.finish();
}

criterion_group!(benches, day09_benchmark);

criterion_main!(benches);

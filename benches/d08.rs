use criterion::{criterion_group, criterion_main, Criterion};

use aoc::d08::{find_lowest_common_z, CycleTracker};

pub fn find_common_cycle(c: &mut Criterion) {
    let mut group = c.benchmark_group("day08");
    group.sample_size(10);
    group.bench_function("common_z", |b| {
        b.iter(|| {
            let mut cycles = [
                CycleTracker {
                    start: "VJP",
                    start_position: 100000,
                    cycle_length: Some(12643),
                    z_count: 1,
                    z_position: Some(1144),
                    multiplier: 0,
                },
                CycleTracker {
                    start: "FNS",
                    start_position: 100000,
                    cycle_length: Some(15871),
                    z_count: 1,
                    z_position: Some(11097),
                    multiplier: 0,
                },
                CycleTracker {
                    start: "FKF",
                    start_position: 100000,
                    cycle_length: Some(19099),
                    z_count: 1,
                    z_position: Some(14594),
                    multiplier: 0,
                },
                CycleTracker {
                    start: "MCB",
                    start_position: 100000,
                    cycle_length: Some(11567),
                    z_count: 1,
                    z_position: Some(4103),
                    multiplier: 0,
                },
                CycleTracker {
                    start: "BQQ",
                    start_position: 100000,
                    cycle_length: Some(19637),
                    z_count: 1,
                    z_position: Some(17822),
                    multiplier: 0,
                },
                CycleTracker {
                    start: "QGF",
                    start_position: 100000,
                    cycle_length: Some(21251),
                    z_count: 1,
                    z_position: Some(6255),
                    multiplier: 0,
                },
            ];
            find_lowest_common_z(&mut cycles);
            assert_eq!(cycles[0].follow_cycle(), 13133452426987);
        })
    });
    group.finish();
}

criterion_group!(benches, find_common_cycle);
criterion_main!(benches);

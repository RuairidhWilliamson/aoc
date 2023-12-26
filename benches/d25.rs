use aoc::d25::parse_components;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn simple_min_cut(c: &mut Criterion) {
    let input = "
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
    ";

    let graph = parse_components(input);
    c.bench_function("simple_min_cut", |b| {
        b.iter(|| graph.clone().simple_min_cut())
    });
}

criterion_group!(benches, simple_min_cut);
criterion_main!(benches);

use gungraun::{library_benchmark, library_benchmark_group, main};

fn read_input(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}

#[library_benchmark]
#[benches::first(args = ["test_input/day04.txt", "input/day04.txt"], setup = read_input)]
fn day04_part2(input: String) {
    aoc::day04::part2(&input);
}

library_benchmark_group!(
    name = days;
    benchmarks = day04_part2
);

main!(library_benchmark_groups = days);

#![allow(clippy::needless_pass_by_value, clippy::exit)]

use gungraun::{library_benchmark, library_benchmark_group, main};

fn read_input(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}

#[library_benchmark]
#[benches::first(args = ["test_input/day01.txt", "input/day01.txt"], setup = read_input)]
fn day01_part1(input: String) {
    aoc::day01::part1(&input);
}

#[library_benchmark]
#[benches::first(args = ["test_input/day01.txt", "input/day01.txt"], setup = read_input)]
fn day01_part2(input: String) {
    aoc::day01::part2(&input);
}

#[library_benchmark]
#[benches::first(args = ["test_input/day02.txt", "input/day02.txt"], setup = read_input)]
fn day02_part1(input: String) {
    aoc::day02::part1(&input);
}

#[library_benchmark]
#[benches::first(args = ["test_input/day02.txt", "input/day02.txt"], setup = read_input)]
fn day02_part2(input: String) {
    aoc::day02::part2(&input);
}

#[library_benchmark]
#[benches::first(args = ["test_input/day03.txt", "input/day03.txt"], setup = read_input)]
fn day03_part1(input: String) {
    aoc::day03::part1(&input);
}

#[library_benchmark]
#[benches::first(args = ["test_input/day03.txt", "input/day03.txt"], setup = read_input)]
fn day03_part2(input: String) {
    aoc::day03::part2(&input);
}

#[library_benchmark]
#[benches::first(args = ["test_input/day04.txt", "input/day04.txt"], setup = read_input)]
fn day04_part1(input: String) {
    aoc::day04::part1(&input);
}

#[library_benchmark]
#[benches::first(args = ["test_input/day04.txt", "input/day04.txt"], setup = read_input)]
fn day04_part2(input: String) {
    aoc::day04::part2(&input);
}

#[library_benchmark]
#[benches::first(args = ["test_input/day05.txt", "input/day05.txt"], setup = read_input)]
fn day05_part1(input: String) {
    aoc::day05::part1(&input);
}

#[library_benchmark]
#[benches::first(args = ["test_input/day05.txt", "input/day05.txt"], setup = read_input)]
fn day05_part2(input: String) {
    aoc::day05::part2(&input);
}

#[library_benchmark]
#[benches::first(args = ["test_input/day06.txt", "input/day06.txt"], setup = read_input)]
fn day06_part1(input: String) {
    aoc::day06::part1(&input);
}

#[library_benchmark]
#[benches::first(args = ["test_input/day06.txt", "input/day06.txt"], setup = read_input)]
fn day06_part2(input: String) {
    aoc::day06::part2(&input);
}

#[library_benchmark]
#[benches::first(args = ["test_input/day07.txt", "input/day07.txt"], setup = read_input)]
fn day07_part1(input: String) {
    aoc::day07::part1(&input);
}

#[library_benchmark]
#[benches::first(args = ["test_input/day07.txt", "input/day07.txt"], setup = read_input)]
fn day07_part2(input: String) {
    aoc::day07::part2(&input);
}

#[library_benchmark]
#[benches::first(args = ["test_input/day08.txt", "input/day08.txt"], setup = read_input)]
fn day08_part1(input: String) {
    aoc::day08::part1(&input);
}

#[library_benchmark]
#[benches::first(args = ["test_input/day08.txt", "input/day08.txt"], setup = read_input)]
fn day08_part2(input: String) {
    aoc::day08::part2(&input);
}

#[library_benchmark]
#[benches::first(args = ["test_input/day09.txt", "input/day09.txt"], setup = read_input)]
fn day09_part1(input: String) {
    aoc::day09::part1(&input);
}

#[library_benchmark]
#[benches::first(args = ["test_input/day09.txt", "input/day09.txt"], setup = read_input)]
fn day09_part2(input: String) {
    aoc::day09::part2(&input);
}

#[library_benchmark]
#[benches::first(args = ["test_input/day10.txt", "input/day10.txt"], setup = read_input)]
fn day10_part1(input: String) {
    aoc::day10::part1(&input);
}

#[library_benchmark]
#[benches::first(args = ["test_input/day10.txt", "input/day10.txt"], setup = read_input)]
fn day10_part2(input: String) {
    aoc::day10::part2(&input);
}

#[library_benchmark]
#[benches::first(args = ["test_input/day11.txt", "input/day11.txt"], setup = read_input)]
fn day11_part1(input: String) {
    aoc::day11::part1(&input);
}

#[library_benchmark]
#[benches::first(args = ["test_input/day11.txt", "input/day11.txt"], setup = read_input)]
fn day11_part2(input: String) {
    aoc::day11::part2(&input);
}

#[library_benchmark]
#[benches::first(args = ["test_input/day12.txt", "input/day12.txt"], setup = read_input)]
fn day12_part1(input: String) {
    aoc::day12::part1(&input);
}

#[library_benchmark]
#[benches::first(args = ["test_input/day12.txt", "input/day12.txt"], setup = read_input)]
fn day12_part2(input: String) {
    aoc::day12::part2(&input);
}

library_benchmark_group!(
    name = days;
    benchmarks =
        day01_part1, day01_part2,
        day02_part1, day02_part2,
        day03_part1, day03_part2,
        day04_part1, day04_part2,
        day05_part1, day05_part2,
        day06_part1, day06_part2,
        day07_part1, day07_part2,
        day08_part1, day08_part2,
        day09_part1, day09_part2,
        day10_part1, day10_part2,
        day11_part1, day11_part2,
        day12_part1, day12_part2,
);

main!(library_benchmark_groups = days);

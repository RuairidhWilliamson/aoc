mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

use std::{path::Path, time::Instant};

fn main() {
    let day = std::env::var("DAY").ok().map(|d| d.parse::<u32>().unwrap());
    let part = std::env::var("PART").ok().map(|p| p.parse::<u8>().unwrap());
    let use_testdata = std::env::var("TESTDATA").is_ok_and(|v| {
        v != "0" && !v.eq_ignore_ascii_case("false") && !v.eq_ignore_ascii_case("no")
    });
    let data_dir = if use_testdata {
        Path::new("test_data")
    } else {
        Path::new("data")
    };

    if day.is_none_or(|d| d == 1) {
        println!("Day 1");
        let input = std::fs::read_to_string(data_dir.join("day01.txt")).unwrap();
        if part.is_none_or(|p| p == 1) {
            let timer = Instant::now();
            println!(
                " Part 1 = {}  Elapsed {:?}",
                day01::part1(&input),
                timer.elapsed()
            );
        }
        if part.is_none_or(|p| p == 2) {
            let timer = Instant::now();
            println!(
                " Part 2 = {}  Elapsed {:?}",
                day01::part2(&input),
                timer.elapsed()
            );
        }
    }
    if day.is_none_or(|d| d == 2) {
        println!("Day 2");
        let input = std::fs::read_to_string(data_dir.join("day02.txt")).unwrap();
        if part.is_none_or(|p| p == 1) {
            let timer = Instant::now();
            println!(
                " Part 1 = {}  Elapsed {:?}",
                day02::part1(&input),
                timer.elapsed()
            );
        }
        if part.is_none_or(|p| p == 2) {
            let timer = Instant::now();
            println!(
                " Part 2 = {}  Elapsed {:?}",
                day02::part2(&input),
                timer.elapsed()
            );
        }
    }
    if day.is_none_or(|d| d == 3) {
        println!("Day 3");
        let input = std::fs::read_to_string(data_dir.join("day03.txt")).unwrap();
        if part.is_none_or(|p| p == 1) {
            let timer = Instant::now();
            println!(
                " Part 1 = {}  Elapsed {:?}",
                day03::part1(&input),
                timer.elapsed()
            );
        }
        if part.is_none_or(|p| p == 2) {
            let timer = Instant::now();
            println!(
                " Part 2 = {}  Elapsed {:?}",
                day03::part2(&input),
                timer.elapsed()
            );
        }
    }
    if day.is_none_or(|d| d == 4) {
        println!("Day 4");
        let input = std::fs::read_to_string(data_dir.join("day04.txt")).unwrap();
        if part.is_none_or(|p| p == 1) {
            let timer = Instant::now();
            println!(
                " Part 1 = {}  Elapsed {:?}",
                day04::part1(&input),
                timer.elapsed()
            );
        }
        if part.is_none_or(|p| p == 2) {
            let timer = Instant::now();
            println!(
                " Part 2 = {}  Elapsed {:?}",
                day04::part2(&input),
                timer.elapsed()
            );
        }
    }
    if day.is_none_or(|d| d == 5) {
        println!("Day 5");
        let input = std::fs::read_to_string(data_dir.join("day05.txt")).unwrap();
        if part.is_none_or(|p| p == 1) {
            let timer = Instant::now();
            println!(
                " Part 1 = {}  Elapsed {:?}",
                day05::part1(&input),
                timer.elapsed()
            );
        }
        if part.is_none_or(|p| p == 2) {
            let timer = Instant::now();
            println!(
                " Part 2 = {}  Elapsed {:?}",
                day05::part2(&input),
                timer.elapsed()
            );
        }
    }
    if day.is_none_or(|d| d == 6) {
        println!("Day 6");
        let input = std::fs::read_to_string(data_dir.join("day06.txt")).unwrap();
        if part.is_none_or(|p| p == 1) {
            let timer = Instant::now();
            println!(
                " Part 1 = {}  Elapsed {:?}",
                day06::part1(&input),
                timer.elapsed()
            );
        }
        if part.is_none_or(|p| p == 2) {
            let timer = Instant::now();
            println!(
                " Part 2 = {}  Elapsed {:?}",
                day06::part2(&input),
                timer.elapsed()
            );
        }
    }
}

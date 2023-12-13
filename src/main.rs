use std::path::Path;

use clap::Parser;

mod common;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;
mod d12;
mod d13;

#[derive(Debug, Parser)]
struct Cli {
    day: Option<usize>,
}

const DAYS: &[fn(&str)] = &[
    d01::run,
    d02::run,
    d03::run,
    d04::run,
    d05::run,
    d06::run,
    d07::run,
    d08::run,
    d09::run,
    d10::run,
    d11::run,
    d12::run,
    d13::run,
];

fn main() {
    let cli = Cli::parse();

    if let Some(day) = cli.day {
        run_day(day);
    } else {
        (1..=DAYS.len()).for_each(|day| {
            println!();
            println!("-- Start Day {day} --");
            run_day(day);
            println!("-- End Day {day} --");
        });
    }
}

fn run_day(day: usize) {
    if !(1..=DAYS.len()).contains(&day) {
        panic!("day {day} out of range");
    }
    let input_path = Path::new("inputs").join(format!("d{day:02}.txt"));
    let input = std::fs::read_to_string(input_path).unwrap();
    DAYS[day - 1](&input);
}

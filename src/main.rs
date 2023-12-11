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

#[derive(Debug, Parser)]
struct Cli {
    day: usize,
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
];

fn main() {
    let cli = Cli::parse();

    let day = cli.day;
    let input_path = Path::new("inputs").join(format!("d{day:02}.txt"));
    let input = std::fs::read_to_string(input_path).unwrap();
    DAYS[day - 1](&input)
}

use std::{io::stdin, path::Path};

use clap::{Parser, ValueEnum};

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
mod d14;
mod d15;
mod d16;
mod d17;
mod d18;

#[derive(Debug, Parser)]
struct Cli {
    day: Option<usize>,
    #[arg(long)]
    stdin: bool,
    #[arg(long)]
    part: Option<Part>,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
enum Part {
    One,
    Two,
}

type PartFn = fn(&str) -> isize;

const DAYS: &[(PartFn, PartFn)] = &[
    d01::PARTS,
    d02::PARTS,
    d03::PARTS,
    d04::PARTS,
    d05::PARTS,
    d06::PARTS,
    d07::PARTS,
    d08::PARTS,
    d09::PARTS,
    d10::PARTS,
    d11::PARTS,
    d12::PARTS,
    d13::PARTS,
    d14::PARTS,
    d15::PARTS,
    d16::PARTS,
    d17::PARTS,
    d18::PARTS,
];

fn main() {
    let cli = Cli::parse();

    if let Some(day) = cli.day {
        if cli.stdin {
            run_day_with_stdin(day, cli.part);
        } else {
            run_day(day, cli.part);
        }
    } else {
        (1..=DAYS.len()).for_each(|day| {
            println!();
            println!("-- Start Day {day} --");
            run_day(day, cli.part);
            println!("-- End Day {day} --");
        });
    }
}

fn run_day(day: usize, part: Option<Part>) {
    if !(1..=DAYS.len()).contains(&day) {
        panic!("day {day} out of range");
    }
    let input_path = Path::new("inputs").join(format!("d{day:02}.txt"));
    let input = std::fs::read_to_string(input_path).unwrap();
    run_day_with_input(day, part, &input);
}

fn run_day_with_stdin(day: usize, part: Option<Part>) {
    if !(1..=DAYS.len()).contains(&day) {
        panic!("day {day} out of range");
    }
    let input = std::io::read_to_string(stdin()).unwrap();
    run_day_with_input(day, part, &input);
}

fn run_day_with_input(day: usize, part: Option<Part>, input: &str) {
    if part != Some(Part::Two) {
        let out = DAYS[day - 1].0(input);
        println!("Part1 answer = {out}");
    }
    if part != Some(Part::One) {
        let out = DAYS[day - 1].1(input);
        println!("Part2 answer = {out}");
    }
}

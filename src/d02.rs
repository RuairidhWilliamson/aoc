#![warn(clippy::unwrap_used)]

use std::{collections::HashMap, num::ParseIntError, str::FromStr};

use thiserror::Error;

use crate::PartFn;

pub const PARTS: (PartFn, PartFn) = (part1, part2);

fn part1(input: &str) -> isize {
    let games: Vec<Game> = input
        .lines()
        .map(|line| line.parse::<Game>())
        .collect::<Result<Vec<Game>, MyError>>()
        .expect("parse games");

    // Part 1
    let mut actual: HashMap<Color, usize> = HashMap::default();
    actual.insert(Color::Red, 12);
    actual.insert(Color::Green, 13);
    actual.insert(Color::Blue, 14);
    let total: usize = games
        .iter()
        .filter(|game| {
            game.sets.iter().all(|s| {
                actual
                    .iter()
                    .all(|(c, max_count)| s.counts.get(c).unwrap_or(&0) <= max_count)
            })
        })
        .map(|game| game.id)
        .sum();
    total as isize
}

fn part2(input: &str) -> isize {
    let games: Vec<Game> = input
        .lines()
        .map(|line| line.parse::<Game>())
        .collect::<Result<Vec<Game>, MyError>>()
        .expect("parse games");
    let power_set_total: usize = games
        .iter()
        .map(|game| {
            let mut max_counts: HashMap<Color, usize> = HashMap::new();
            game.sets.iter().for_each(|s| {
                s.counts.iter().for_each(|(&c, &count)| {
                    let max = max_counts.entry(c).or_default();
                    *max = count.max(*max);
                });
            });
            max_counts.iter().map(|(_, &c)| c).product::<usize>()
        })
        .sum();
    power_set_total as isize
}

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<GameSet>,
}

impl FromStr for Game {
    type Err = MyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("Game ").ok_or(MyError::MissingGamePrefix)?;
        let (id, rest) = s.split_once(':').ok_or(MyError::MissingColon)?;
        let sets = rest
            .split(';')
            .map(|s| s.parse())
            .collect::<Result<Vec<GameSet>, MyError>>()?;
        Ok(Game {
            id: id.parse()?,
            sets,
        })
    }
}

#[derive(Debug)]
struct GameSet {
    counts: HashMap<Color, usize>,
}

impl FromStr for GameSet {
    type Err = MyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut counts = HashMap::default();
        for c in s.split(',') {
            let (count, color) = c.trim().split_once(' ').ok_or(MyError::MissingSpace)?;
            let color: Color = color.parse()?;
            let e = counts.entry(color).or_default();
            *e += count.parse::<usize>()?;
        }
        Ok(GameSet { counts })
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = MyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            c => Err(MyError::UnknownColor(c.to_owned())),
        }
    }
}

#[derive(Debug, Error)]
enum MyError {
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Unknown color: {0}")]
    UnknownColor(String),
    #[error("Invalid int: {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("Missing space in color count")]
    MissingSpace,
    #[error("Missing colon in game")]
    MissingColon,
    #[error("Missing game prefix")]
    MissingGamePrefix,
}

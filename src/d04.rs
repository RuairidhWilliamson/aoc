use std::{num::ParseIntError, str::FromStr};

use crate::PartFn;

pub const PARTS: (PartFn, PartFn) = (part1, part2);

fn part1(input: &str) -> isize {
    let cards = input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<Card>, MyError>>()
        .expect("parse cards");
    let total: usize = cards
        .iter()
        .map(|c| {
            let win_count = c.win_count();
            if win_count == 0 {
                0
            } else {
                1 << (win_count - 1)
            }
        })
        .sum();
    total as isize
}

fn part2(input: &str) -> isize {
    let cards = input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<Card>, MyError>>()
        .expect("parse cards");
    let mut cards: Vec<_> = cards
        .into_iter()
        .map(|c| CardAcc {
            card: c,
            instances: 1,
        })
        .collect();

    (0..cards.len()).for_each(|i| {
        let c = &cards[i];
        let win_count = c.card.win_count();
        let instances = c.instances;
        let copied_cards = &mut cards[i + 1..i + 1 + win_count];
        copied_cards
            .iter_mut()
            .for_each(|c| c.instances += instances);
    });
    let card_count: usize = cards.iter().map(|c| c.instances).sum();
    card_count as isize
}

#[derive(Debug)]
struct CardAcc {
    card: Card,
    instances: usize,
}

#[derive(Debug)]
struct Card {
    #[allow(dead_code)]
    id: usize,
    winning: Vec<usize>,
    numbers: Vec<usize>,
}

impl Card {
    fn win_count(&self) -> usize {
        self.numbers
            .iter()
            .filter(|n| self.winning.contains(n))
            .count()
    }
}

impl FromStr for Card {
    type Err = MyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rest = s.strip_prefix("Card ").ok_or(MyError::MissingCardPrefix)?;
        let (id, rest) = rest.split_once(':').ok_or(MyError::MissingColon)?;
        let id = id.trim().parse()?;
        let (winning, numbers) = rest.split_once('|').ok_or(MyError::MissingSeparator)?;
        let winning = winning
            .trim()
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse())
            .collect::<Result<Vec<usize>, ParseIntError>>()?;
        let numbers = numbers
            .trim()
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse())
            .collect::<Result<Vec<usize>, ParseIntError>>()?;
        Ok(Self {
            id,
            winning,
            numbers,
        })
    }
}

#[derive(Debug, thiserror::Error)]
enum MyError {
    #[error("missing card prefix")]
    MissingCardPrefix,
    #[error("missing colon")]
    MissingColon,
    #[error("missing separator")]
    MissingSeparator,
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("invalid int: {0}")]
    ParseInt(#[from] ParseIntError),
}

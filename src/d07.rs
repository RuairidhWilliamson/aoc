use std::str::FromStr;

use crate::PartFn;

pub const PARTS: (PartFn, PartFn) = (part1, part2);

fn part1(_input: &str) -> isize {
    0
}

fn part2(input: &str) -> isize {
    let mut hands: Vec<_> = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let hand: Hand = hand.parse().unwrap();
            let bid = bid.parse::<usize>().unwrap();
            HandWithBid { hand, bid }
        })
        .collect();
    hands.sort_unstable_by(|a, b| a.hand.cmp(&b.hand));
    // hands
    //     .iter()
    //     .for_each(|h| println!("{} {:?}", h.hand, h.hand.hand_type()));
    let total: usize = hands.iter().enumerate().map(|(i, h)| (i + 1) * h.bid).sum();
    total as isize
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl Card {
    fn parse(c: char) -> Self {
        match c {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Joker,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!("cannot parse: {c}"),
        }
    }

    fn as_char(&self) -> char {
        match self {
            Self::Two => '2',
            Self::Three => '3',
            Self::Four => '4',
            Self::Five => '5',
            Self::Six => '6',
            Self::Seven => '7',
            Self::Eight => '8',
            Self::Nine => '9',
            Self::Ten => 'T',
            Self::Joker => 'J',
            Self::Queen => 'Q',
            Self::King => 'K',
            Self::Ace => 'A',
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    High,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hand: Vec<_> = s
            .as_bytes()
            .iter()
            .map(|&x| Card::parse(x as char))
            .collect();
        Ok(Self {
            cards: hand.try_into().unwrap(),
        })
    }
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.cards.iter().map(|c| c.as_char()).collect();
        f.write_str(&s)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let ord = self.hand_type().cmp(&other.hand_type());
        if std::cmp::Ordering::Equal != ord {
            return ord;
        };
        self.cards.cmp(&other.cards)
    }
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut unique = self.cards.to_vec();
        unique.sort_unstable();
        unique.dedup();
        let counts: Vec<(Card, usize)> = unique
            .into_iter()
            .filter(|c| !matches!(c, Card::Joker))
            .map(|u| (u, self.cards.iter().filter(|&c| *c == u).count()))
            .collect();
        let joker_count: usize = self.cards.iter().filter(|&c| *c == Card::Joker).count();
        // println!("{counts:?} {joker_count:?}");
        if joker_count == 5 || counts.iter().any(|(_, count)| *count + joker_count == 5) {
            HandType::FiveOfAKind
        } else if counts.iter().any(|(_, count)| *count + joker_count == 4) {
            HandType::FourOfAKind
        } else if self.is_fullhouse(&counts, joker_count) {
            HandType::FullHouse
        } else if counts.iter().any(|(_, count)| *count + joker_count == 3) {
            HandType::ThreeOfAKind
        } else if counts.iter().filter(|(_, count)| *count == 2).count() == 2 {
            HandType::TwoPair
        } else if counts.iter().any(|(_, count)| *count + joker_count == 2) {
            HandType::Pair
        } else {
            HandType::High
        }
    }

    fn is_fullhouse(&self, counts: &[(Card, usize)], joker_count: usize) -> bool {
        let Some((card, _)) = counts.iter().find(|(_, count)| *count + joker_count == 3) else {
            return false;
        };
        counts.iter().any(|(c, count)| c != card && *count == 2)
    }
}

#[derive(Debug)]
struct HandWithBid {
    hand: Hand,
    bid: usize,
}

#[cfg(test)]
mod tests {
    use super::{Hand, HandType};

    #[test]
    fn hand_type_order() {
        assert!(HandType::Pair < HandType::TwoPair);
        assert!(HandType::High < HandType::TwoPair);
    }

    #[test]
    fn jokers() {
        let hand: Hand = "AAKJK".parse().unwrap();
        assert_eq!(hand.hand_type(), HandType::FullHouse);
        let hand: Hand = "AAKKK".parse().unwrap();
        assert_eq!(hand.hand_type(), HandType::FullHouse);
        let hand: Hand = "AKQJT".parse().unwrap();
        assert_eq!(hand.hand_type(), HandType::Pair);
        let hand: Hand = "AAQJT".parse().unwrap();
        assert_eq!(hand.hand_type(), HandType::ThreeOfAKind);
    }

    #[test]
    fn joker_two_pair() {
        let hand: Hand = "AJJQT".parse().unwrap();
        assert_eq!(hand.hand_type(), HandType::ThreeOfAKind);
        let hand: Hand = "JAQT9".parse().unwrap();
        assert_eq!(hand.hand_type(), HandType::Pair);
        let hand: Hand = "AAQQT".parse().unwrap();
        assert_eq!(hand.hand_type(), HandType::TwoPair);
        let hand: Hand = "Q2AT9".parse().unwrap();
        assert_eq!(hand.hand_type(), HandType::High);
        let hand: Hand = "JJJJJ".parse().unwrap();
        assert_eq!(hand.hand_type(), HandType::FiveOfAKind);
    }

    #[test]
    fn hand_order() {
        assert!("AAAAA".parse::<Hand>() > "JJJJJ".parse::<Hand>());
    }
}

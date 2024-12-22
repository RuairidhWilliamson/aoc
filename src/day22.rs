use std::collections::{HashMap, HashSet};

pub fn solve_part1(input: &str) -> usize {
    parse_initials(input).map(|s| calc_n(s, 2000)).sum()
}

pub fn solve_part2(input: &str) -> u32 {
    let mut seqs = HashMap::new();
    parse_initials(input).for_each(|s| seq_map(s, 2001, &mut seqs));
    seqs.into_values().max().unwrap()
}

fn parse_initials(input: &str) -> impl Iterator<Item = usize> + use<'_> {
    input.lines().map(|l| l.parse().unwrap())
}

fn calc_next(mut s: usize) -> usize {
    s = ((s * 64) ^ s) % 16777216;
    s = ((s / 32) ^ s) % 16777216;
    s = ((s * 2048) ^ s) % 16777216;
    s
}

fn calc_n(initial: usize, n: usize) -> usize {
    let mut s = initial;
    for _ in 0..n {
        s = calc_next(s);
    }
    s
}

fn prices(mut s: usize, n: usize) -> impl Iterator<Item = i8> {
    (0..n).map(move |_| {
        let price = (s % 10) as i8;
        s = calc_next(s);
        price
    })
}

type Sequence = [i8; 4];

fn seq_map(s: usize, n: usize, seqs: &mut HashMap<Sequence, u32>) {
    let mut already_sold = HashSet::<Sequence>::new();
    let prices: Vec<i8> = prices(s, n).collect();
    for w in prices.windows(5) {
        let sequence = [0, 1, 2, 3].map(|i| w[i + 1] - w[i]);
        if already_sold.insert(sequence) {
            *seqs.entry(sequence).or_default() += w[4] as u32;
        }
    }
}

#[cfg(test)]
const INPUT: &str = "1
10
100
2024";

#[test]
fn practice_part1() {
    assert_eq!(solve_part1(INPUT), 37327623);
}

#[test]
fn secret_numbers() {
    assert_eq!(calc_next(123), 15887950);
}

#[test]
fn prices_test() {
    assert_eq!(
        prices(123, 10).collect::<Vec<_>>(),
        vec![3, 0, 6, 5, 4, 4, 6, 4, 4, 2]
    );
    let mut seqs = HashMap::new();
    seq_map(123, 10, &mut seqs);
    assert_eq!(*seqs.get(&[-1, -1, 0, 2]).unwrap(), 6);
    assert_eq!(*seqs.get(&[-3, 6, -1, -1]).unwrap(), 4);
}

#[test]
fn practice_part2() {
    assert_eq!(solve_part2("1\n2\n3\n2024"), 23);
}

pub fn solve_part1(input: &str) -> usize {
    parse_initials(input).map(|s| calc_n(s, 2000)).sum()
}

pub fn solve_part2(input: &str) -> u32 {
    let mut seqs = SequenceMap::new(0);
    let mut already_sold = SequenceMap::new(false);
    parse_initials(input).for_each(|s| seq_map(s, 2001, &mut seqs, &mut already_sold));
    *seqs.d.iter().max().unwrap()
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

fn prices_collect(s: usize, n: usize) -> Vec<i8> {
    prices(s, n).collect()
}

type Sequence = [i8; 4];

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct SequenceHash(u32);

impl SequenceHash {
    const MAX: SequenceHash = Self::new([9, 9, 9, 9]);

    const fn new(seq: Sequence) -> Self {
        Self(
            ((seq[0] + 10) as u32) << 15
                | ((seq[1] + 10) as u32) << 10
                | ((seq[2] + 10) as u32) << 5
                | ((seq[3] + 10) as u32),
        )
    }
}

struct SequenceMap<T> {
    d: Box<[T]>,
}

impl<T: Copy> SequenceMap<T> {
    fn new(default: T) -> Self {
        Self {
            d: (0..SequenceHash::MAX.0).map(|_| default).collect(),
        }
    }

    fn set_all(&mut self, value: T) {
        self.d.fill(value);
    }

    #[allow(unused)]
    fn get(&self, s: SequenceHash) -> &T {
        self.d.get(s.0 as usize).unwrap()
    }

    fn get_mut(&mut self, s: SequenceHash) -> &mut T {
        self.d.get_mut(s.0 as usize).unwrap()
    }
}

impl SequenceMap<bool> {
    fn insert(&mut self, s: SequenceHash) -> bool {
        let b = self.get_mut(s);
        if *b {
            false
        } else {
            *b = true;
            true
        }
    }
}

fn seq_map(s: usize, n: usize, seqs: &mut SequenceMap<u32>, already_sold: &mut SequenceMap<bool>) {
    already_sold.set_all(false);

    let prices: Vec<i8> = prices_collect(s, n);
    for w in prices.windows(5) {
        let sequence = SequenceHash::new([0, 1, 2, 3].map(|i| w[i + 1] - w[i]));
        if already_sold.insert(sequence) {
            *seqs.get_mut(sequence) += w[4] as u32;
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
    let mut seqs = SequenceMap::new(0);
    let mut already_sold = SequenceMap::new(false);
    seq_map(123, 10, &mut seqs, &mut already_sold);
    assert_eq!(*seqs.get(SequenceHash::new([-1, -1, 0, 2])), 6);
    assert_eq!(*seqs.get(SequenceHash::new([-3, 6, -1, -1])), 4);
}

#[test]
fn practice_part2() {
    assert_eq!(solve_part2("1\n2\n3\n2024"), 23);
}

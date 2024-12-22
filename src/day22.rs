pub fn solve_part1(input: &str) -> usize {
    parse_initials(input).map(|s| calc_n(s, 2000)).sum()
}

pub fn solve_part2(input: &str) -> u32 {
    let mut seqs = SequenceMap::new(0);
    let mut already_sold = AlreadySoldPass::new();
    for s in parse_initials(input) {
        seq_map::<2001>(s, &mut seqs, &mut already_sold)
    }
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

fn prices<const N: usize>(mut s: usize) -> impl Iterator<Item = i8> {
    (0..N).map(move |_| {
        let price = (s % 10) as i8;
        s = calc_next(s);
        price
    })
}

type Sequence = [i8; 4];

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct SequenceHash(u32);

impl SequenceHash {
    const MAX: SequenceHash = Self::new([9, 9, 9, 9]);

    const fn new(seq: Sequence) -> Self {
        debug_assert!(seq[0] < 10);
        debug_assert!(seq[1] < 10);
        debug_assert!(seq[2] < 10);
        debug_assert!(seq[3] < 10);
        debug_assert!(seq[0] > -10);
        debug_assert!(seq[1] > -10);
        debug_assert!(seq[2] > -10);
        debug_assert!(seq[3] > -10);
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

    fn get_mut(&mut self, s: SequenceHash) -> &mut T {
        &mut self.d[s.0 as usize]
    }
}

struct AlreadySoldPass {
    record: SequenceMap<u32>,
    index: u32,
}

impl AlreadySoldPass {
    fn new() -> Self {
        Self {
            record: SequenceMap::new(0),
            index: 0,
        }
    }

    fn insert(&mut self, s: SequenceHash) -> bool {
        let existing = self.record.get_mut(s);
        if *existing == self.index {
            false
        } else {
            *existing = self.index;
            true
        }
    }
}

fn seq_map<const N: usize>(
    s: usize,
    seqs: &mut SequenceMap<u32>,
    already_sold: &mut AlreadySoldPass,
) {
    already_sold.index += 1;

    let mut prices = prices::<N>(s);
    let mut prevs: [i8; 4] = [
        prices.next().unwrap(),
        prices.next().unwrap(),
        prices.next().unwrap(),
        prices.next().unwrap(),
    ];
    for p in prices {
        let sequence = SequenceHash::new([
            prevs[1] - prevs[0],
            prevs[2] - prevs[1],
            prevs[3] - prevs[2],
            p - prevs[3],
        ]);
        if already_sold.insert(sequence) {
            *seqs.get_mut(sequence) += p as u32;
        }
        prevs[0] = prevs[1];
        prevs[1] = prevs[2];
        prevs[2] = prevs[3];
        prevs[3] = p;
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
fn practice_part2() {
    assert_eq!(solve_part2("1\n2\n3\n2024"), 23);
}

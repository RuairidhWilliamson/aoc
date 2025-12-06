use std::ops::RangeInclusive;

pub fn part1(input: &str) -> u64 {
    let ranges = input.trim().split(',').map(|rng| {
        let (start, end) = rng.split_once('-').unwrap();
        (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
    });

    ranges
        .map(|(start, end)| sum_invalid_ids(start..=end, factors_only_2, false))
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let ranges = input.trim().split(',').map(|rng| {
        let (start, end) = rng.split_once('-').unwrap();
        (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
    });

    ranges
        .map(|(start, end)| sum_invalid_ids(start..=end, factors_of, true))
        .sum()
}

fn pow10(n: u32) -> u64 {
    let mut x = 1;
    for _ in 0..n {
        x *= 10;
    }
    x
}

fn digits_of(n: u64) -> u32 {
    n.ilog10() + 1
}

fn factors_only_2(n: u32) -> impl Iterator<Item = u32> {
    std::iter::once(2).filter(move |_| n % 2 == 0)
}

fn factors_of(n: u32) -> impl Iterator<Item = u32> {
    (2..=n).filter(move |x| n % x == 0)
}

fn is_invalid_id<F, I>(id: u64, factor_fn: F) -> bool
where
    F: Fn(u32) -> I + Copy,
    I: Iterator<Item = u32>,
{
    let digits = digits_of(id);
    'outer: for len in factor_fn(digits) {
        let denom = pow10(digits / len);
        let mut id = id;
        let rem = id % denom;
        for _ in 1..len {
            id = id / denom;
            if id % denom != rem {
                continue 'outer;
            }
        }
        return true;
    }
    false
}

fn sum_invalid_ids<F, I>(rng: RangeInclusive<u64>, factor_fn: F, dedup: bool) -> u64
where
    F: Fn(u32) -> I + Copy,
    I: Iterator<Item = u32>,
{
    let (start, end) = rng.clone().into_inner();
    let start_digits = digits_of(start);
    let end_digits = digits_of(end);
    if start_digits == end_digits {
        let digits = start_digits;
        return factor_fn(digits)
            .map(|len| {
                let mult = pow10(digits / len);
                let x = pow10(digits - digits / len);
                (start / x..=end / x)
                    .filter(|left| !dedup || !is_invalid_id(*left, factor_fn))
                    .map(|left| {
                        let mut id = left;
                        for _ in 1..len {
                            id = id * mult + left;
                        }
                        id
                    })
                    .filter(|id| start <= *id && *id <= end)
                    .sum::<u64>()
            })
            .sum::<u64>();
    }
    if start_digits + 1 == end_digits {
        let x = pow10(start_digits);
        return sum_invalid_ids(start..=x - 1, factor_fn, dedup)
            + sum_invalid_ids(x..=end, factor_fn, dedup);
    }
    // Fallback brute force
    rng.filter(|id| is_invalid_id(*id, factor_fn)).sum()
}

#[test]
fn test_is_invalid_part2() {
    assert!(is_invalid_id(565656, factors_of));
}

#[test]
fn test_digits_of() {
    assert_eq!(digits_of(9), 1);
    assert_eq!(digits_of(10), 2);
    assert_eq!(digits_of(11), 2);
}

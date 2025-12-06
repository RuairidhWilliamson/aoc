pub fn part1(input: &str) -> u64 {
    let ranges = input.trim().split(',').map(|rng| {
        let (start, end) = rng.split_once('-').unwrap();
        (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
    });

    ranges
        .map(|(start, end)| {
            (start..=end)
                .filter(|id| is_invalid_id_part1(*id))
                .sum::<u64>()
        })
        .sum()
}

fn is_invalid_id_part1(id: u64) -> bool {
    let digits = digits_of(id);
    if digits % 2 != 0 {
        return false;
    }
    let mut x = 1;
    for _ in 0..(digits / 2) {
        x *= 10;
    }
    id / x == id % x
}

#[test]
fn test_is_invalid_part1() {
    assert!(is_invalid_id_part1(123123));
}

fn digits_of(n: u64) -> u32 {
    n.ilog10() + 1
}

fn factors_of(n: u32) -> impl Iterator<Item = u32> {
    (2..=n).filter(move |x| n % x == 0)
}

fn is_invalid_id_part2(id: u64) -> bool {
    let digits = digits_of(id);
    'outer: for len in factors_of(digits) {
        let denom = 10u64.pow(digits / len);
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

fn find_invalid_ids_part2(rng: std::ops::RangeInclusive<u64>) -> impl Iterator<Item = u64> {
    rng.filter(|id| is_invalid_id_part2(*id))
}

pub fn part2(input: &str) -> u64 {
    let ranges = input.trim().split(',').map(|rng| {
        let (start, end) = rng.split_once('-').unwrap();
        (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
    });

    ranges
        .map(|(start, end)| find_invalid_ids_part2(start..=end).sum::<u64>())
        .sum()
}

#[test]
fn test_is_invalid_part2() {
    assert!(is_invalid_id_part2(565656));
}

pub fn part1(input: &str) -> u64 {
    fn is_invalid_id(id: u64) -> bool {
        let x = 10u64.pow((id.ilog10() + 1) / 2);
        id / x == id % x
    }
    let ranges = input.trim().split(',').map(|rng| {
        let (start, end) = rng.split_once('-').unwrap();
        (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
    });

    ranges
        .map(|(start, end)| (start..=end).filter(|id| is_invalid_id(*id)).sum::<u64>())
        .sum()
}

fn factors_of(n: u32) -> impl Iterator<Item = u32> {
    (2..=n).filter(move |x| n % x == 0)
}

fn is_invalid_id_part2(id: u64) -> bool {
    let digits = id.ilog10() + 1;
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

pub fn part2(input: &str) -> u64 {
    let ranges = input.trim().split(',').map(|rng| {
        let (start, end) = rng.split_once('-').unwrap();
        (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
    });

    ranges
        .map(|(start, end)| {
            (start..=end)
                .filter(|id| is_invalid_id_part2(*id))
                // .inspect(|id| eprintln!("{id}"))
                .sum::<u64>()
        })
        .sum()
}

#[test]
fn test_is_invalid_part2() {
    assert!(is_invalid_id_part2(565656));
}

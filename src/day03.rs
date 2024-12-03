use std::sync::LazyLock;

pub fn solve_part1(input: &str) -> usize {
    SIMPLE_REGEX
        .captures_iter(input)
        .map(|caps| {
            let lhs: usize = caps.get(1).unwrap().as_str().parse().unwrap();
            let rhs: usize = caps.get(2).unwrap().as_str().parse().unwrap();
            lhs * rhs
        })
        .sum()
}

pub fn solve_part2(input: &str) -> usize {
    let mut mul_enabled = true;
    COMPLEX_REGEX
        .captures_iter(input)
        .map(|caps| {
            let all = caps.get(0).unwrap().as_str();
            if all.starts_with("mul(") {
                if mul_enabled {
                    let lhs: usize = caps.get(1).unwrap().as_str().parse().unwrap();
                    let rhs: usize = caps.get(2).unwrap().as_str().parse().unwrap();
                    lhs * rhs
                } else {
                    0
                }
            } else if all == "do()" {
                mul_enabled = true;
                0
            } else if all == "don't()" {
                mul_enabled = false;
                0
            } else {
                unreachable!()
            }
        })
        .sum()
}

static SIMPLE_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").expect("compile simple regex")
});

static COMPLEX_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|do\(\)|don't\(\)")
        .expect("compile simple regex")
});

#[cfg(test)]
const INPUT1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

#[cfg(test)]
const INPUT2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

#[test]
fn practice_part1() {
    assert_eq!(solve_part1(INPUT1), 161);
}

#[test]
fn practice_part2() {
    assert_eq!(solve_part2(INPUT2), 48);
}

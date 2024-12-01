use std::iter::Peekable;

pub fn solve_part1(input: &str) -> usize {
    let (mut left, mut right): (Vec<usize>, Vec<usize>) =
        input.lines().map(|line| parse_line(line).unwrap()).unzip();
    left.sort_unstable();
    right.sort_unstable();
    left.into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

pub fn solve_part2(input: &str) -> usize {
    let (mut left, mut right): (Vec<usize>, Vec<usize>) =
        input.lines().map(|line| parse_line(line).unwrap()).unzip();
    left.sort_unstable();
    right.sort_unstable();

    debug_assert_eq!(left.len(), right.len());

    let mut left = left.into_iter().peekable();
    let mut right = right.into_iter().peekable();

    let mut similarity = 0;

    while let Some(s) = inner_loop(&mut left, &mut right) {
        similarity += s;
    }

    similarity
}

fn inner_loop<I>(left: &mut Peekable<I>, right: &mut Peekable<I>) -> Option<usize>
where
    I: Iterator<Item = usize>,
{
    let mut left_value = left.next()?;
    let mut right_value = right.next()?;

    loop {
        match left_value.cmp(&right_value) {
            std::cmp::Ordering::Less => {
                left_value = left.next()?;
            }
            std::cmp::Ordering::Greater => {
                right_value = right.next()?;
            }
            std::cmp::Ordering::Equal => break,
        }
    }
    debug_assert_eq!(left_value, right_value);
    let left_count = count_matching_values(left, left_value);
    let right_count = count_matching_values(right, right_value);
    Some(left_value * left_count * right_count)
}

fn count_matching_values<I>(iter: &mut Peekable<I>, expected: usize) -> usize
where
    I: Iterator<Item = usize>,
{
    let mut count = 1;
    while iter.next_if_eq(&expected).is_some() {
        count += 1;
    }
    count
}

fn parse_line(line: &str) -> Option<(usize, usize)> {
    let mut iter = line.split(' ').filter(|e| !e.is_empty());
    let first = iter.next()?.parse().ok()?;
    let last = iter.next_back()?.parse().ok()?;
    Some((first, last))
}

#[test]
fn practice_part1() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3";
    let answer = solve_part1(input);
    assert_eq!(answer, 11);
}

#[test]
fn practice_part2() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3";
    let answer = solve_part2(input);
    assert_eq!(answer, 31);
}

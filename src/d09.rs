use crate::PartFn;

pub const PARTS: (PartFn, PartFn) = (part1, part2);

fn part1(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            let sequence = parse_sequence(line);
            next_element(&sequence)
        })
        .sum()
}

fn part2(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            let sequence = parse_sequence(line);
            prev_element(&sequence)
        })
        .sum()
}

fn parse_sequence(sequence: &str) -> Vec<isize> {
    sequence.split(' ').map(|x| x.parse().unwrap()).collect()
}

fn next_element(sequence: &[isize]) -> isize {
    // println!("{sequence:?}");
    if all_zero(sequence) {
        return 0;
    }
    let next_diff = next_element(&differences(sequence));
    let next = sequence.last().unwrap() + next_diff;
    // println!("{sequence:?} {next}");
    next
}

fn prev_element(sequence: &[isize]) -> isize {
    if all_zero(sequence) {
        return 0;
    }
    let prev_diff = prev_element(&differences(sequence));
    let prev = sequence.first().unwrap() - prev_diff;
    prev
}

fn differences(sequence: &[isize]) -> Vec<isize> {
    assert!(sequence.len() > 1);
    sequence
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, x)| (sequence[i - 1], x))
        .map(|(a, b)| b - a)
        .collect()
}

fn all_zero(sequence: &[isize]) -> bool {
    sequence.iter().all(|x| *x == 0)
}

#[cfg(test)]
mod tests {
    use super::{next_element, prev_element};

    #[test]
    fn all_zeros() {
        let e = next_element(&[0, 0, 0, 0, 0]);
        assert_eq!(e, 0);
    }

    #[test]
    fn linear() {
        let e = next_element(&[0, 1, 2, 3, 4]);
        assert_eq!(e, 5);
    }

    #[test]
    fn linear_prev() {
        let e = prev_element(&[2, 3, 4]);
        assert_eq!(e, 1);
    }
}

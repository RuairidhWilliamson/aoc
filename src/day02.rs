pub fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(parse_report)
        .map(report_deltas)
        .map(is_report_safe)
        .filter(|x| *x)
        .count()
}

pub fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .map(parse_report)
        .map(is_report_safe_damp)
        .filter(|x| *x)
        .count()
}

fn parse_report(line: &str) -> impl Iterator<Item = usize> + use<'_> {
    line.split(' ').map(|x| x.parse().unwrap())
}

fn pairs<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = (T, T)>
where
    T: Copy + 'static,
{
    let mut prev = None;
    iter.filter_map(move |x| {
        let returning_prev = prev.take();
        prev = Some(x);
        Some((returning_prev?, x))
    })
}

fn report_deltas(report: impl Iterator<Item = usize>) -> impl Iterator<Item = isize> {
    pairs(report.map(|x| isize::try_from(x).unwrap())).map(|(a, b)| a - b)
}

fn is_report_safe(mut deltas: impl Iterator<Item = isize>) -> bool {
    let mut sign = None;
    deltas.all(|d| {
        if let Some(s) = sign {
            if d.signum() != s {
                return false;
            }
        } else {
            sign = Some(d.signum());
        }
        1 <= d.abs() && d.abs() <= 3
    })
}

fn is_report_safe_damp(report: impl Iterator<Item = usize>) -> bool {
    let report: Vec<usize> = report.collect();
    let mut direction = 0;
    for i in 1..report.len() {
        let a = report[i - 1] as isize;
        let b = report[i] as isize;
        let d = a - b;
        direction += d.signum();
        if !(1..=3).contains(&d) || direction == 0 || direction.signum() != d.signum() {
            // Probably an error here
            if is_report_safe_excluding(report.iter().copied(), i - 1) {
                return true;
            }
            // Probably an error here
            if is_report_safe_excluding(report.iter().copied(), i) {
                return true;
            }
            // Otherwise try everything
            for i in 0..report.len() {
                if is_report_safe_excluding(report.iter().copied(), i) {
                    return true;
                }
            }
            return false;
        }
    }
    true
}

fn is_report_safe_excluding(report: impl Iterator<Item = usize>, exclude_index: usize) -> bool {
    is_report_safe(report_deltas(
        report
            .enumerate()
            .filter(|(i, _)| *i != exclude_index)
            .map(|(_, x)| x),
    ))
}

#[cfg(test)]
const PRACTICE_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

#[test]
fn practice_part1() {
    assert_eq!(solve_part1(PRACTICE_INPUT), 2);
}

#[test]
fn practice_part2() {
    assert_eq!(solve_part2(PRACTICE_INPUT), 4);
}

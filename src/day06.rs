pub fn part1(input: &str) -> usize {
    let lines: Vec<&[u8]> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.as_bytes())
        .collect();
    let line_length = lines[0].len();
    let mut grand_total = 0;
    let mut width = 0;
    for i in (0..line_length).rev() {
        width += 1;
        match lines[lines.len() - 1][i] {
            b' ' => {}
            b'*' => {
                grand_total += (0..(lines.len() - 1))
                    .map(|j| parse_usize_iter(lines[j][i..i + width].iter().copied()).unwrap())
                    .product::<usize>();
                width = 0;
            }
            b'+' => {
                grand_total += (0..(lines.len() - 1))
                    .map(|j| parse_usize_iter(lines[j][i..i + width].iter().copied()).unwrap())
                    .sum::<usize>();
                width = 0;
            }
            c => panic!("unexpected {c}"),
        }
    }
    grand_total
}

pub fn part2(input: &str) -> usize {
    let lines: Vec<&[u8]> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.as_bytes())
        .collect();
    let line_length = lines[0].len();
    let mut grand_total = 0;
    let mut numbers = Vec::new();
    for i in (0..line_length).rev() {
        let iter = (0..(lines.len() - 1)).map(|j| lines[j][i]);
        let Some(numb) = parse_usize_iter(iter) else {
            continue;
        };
        numbers.push(numb);
        match lines[lines.len() - 1][i] {
            b' ' => {}
            b'*' => {
                grand_total += numbers.iter().product::<usize>();
                numbers.clear();
            }
            b'+' => {
                grand_total += numbers.iter().sum::<usize>();
                numbers.clear();
            }
            c => panic!("unexpected {c}"),
        }
    }
    grand_total
}

fn parse_usize_iter(iter: impl Iterator<Item = u8>) -> Option<usize> {
    let mut out: usize = 0;
    let mut found_digit = false;
    for digit in iter {
        if digit.is_ascii_digit() {
            found_digit = true;
            out = out * 10 + (digit - b'0') as usize;
        }
    }
    if found_digit { Some(out) } else { None }
}

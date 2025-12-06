pub fn part1(input: &str) -> usize {
    let mut number_lists: Vec<Vec<usize>> = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let first_char = line.chars().next().unwrap();
        if first_char.is_digit(10) || first_char.is_whitespace() {
            for (i, n) in line
                .split(' ')
                .filter(|n| !n.is_empty())
                .map(|n| n.parse::<usize>().unwrap())
                .enumerate()
            {
                while i >= number_lists.len() {
                    number_lists.push(Vec::new());
                }
                number_lists[i].push(n);
            }
        } else {
            // last row
            let mut grand_total: usize = 0;
            for (i, sym) in line.split(' ').filter(|n| !n.is_empty()).enumerate() {
                match sym {
                    "*" => {
                        grand_total += number_lists[i].iter().product::<usize>();
                    }
                    "+" => {
                        grand_total += number_lists[i].iter().sum::<usize>();
                    }
                    _ => panic!("found bad operator symbol = {}", sym),
                }
            }
            return grand_total;
        }
    }
    panic!("missing operator row")
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
    let numb: Vec<u8> = iter.collect();
    let numb_str = str::from_utf8(&numb).unwrap().trim();
    if numb_str.is_empty() {
        return None;
    }
    let numb: usize = numb_str.parse().unwrap();
    Some(numb)
}

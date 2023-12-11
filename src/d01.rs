use std::char;

const NUMBERS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn run(input: &str) {
    let total: usize = input
        .lines()
        .map(|line| {
            let a_index = match_index_forward(line);
            let b_index = match_index_backward(line);
            let a = convert_number(line, a_index);
            let b = convert_number(line, b_index);
            a * 10 + b
        })
        .sum();
    println!("Total = {total}");
}

fn match_index_forward(line: &str) -> usize {
    let numerical_index = line.find(|c: char| c.is_ascii_digit());
    NUMBERS
        .iter()
        .map(|n| line.find(n))
        .chain(std::iter::once(numerical_index))
        .flatten()
        .min()
        .unwrap()
}

fn match_index_backward(line: &str) -> usize {
    let numerical_index = line.rfind(|c: char| c.is_ascii_digit());
    NUMBERS
        .iter()
        .map(|n| line.rfind(n))
        .chain(std::iter::once(numerical_index))
        .flatten()
        .max()
        .unwrap()
}

fn convert_number(line: &str, index: usize) -> usize {
    let n = line.as_bytes()[index];
    if n.is_ascii_digit() {
        return (n - b'0') as usize;
    }
    for (i, &num) in NUMBERS.iter().enumerate() {
        if index + num.len() > line.len() {
            continue;
        }
        if &line[index..index + num.len()] == num {
            return i + 1;
        }
    }
    panic!("no match")
}

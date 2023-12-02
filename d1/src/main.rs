use std::{char, io::stdin};

const NUMBERS: &[&'static str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let stdin = stdin();
    let total: usize = stdin
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let a_index = match_index_forward(&line);
            let b_index = match_index_backward(&line);
            let a = convert_number(&line, a_index);
            let b = convert_number(&line, b_index);
            let n = a as usize * 10 + b as usize;
            println!("{n}");
            n
        })
        .sum();
    println!("Total = {total}");
}

fn match_index_forward(line: &str) -> usize {
    let numerical_index = line.find(|c: char| c.is_ascii_digit());
    NUMBERS
        .into_iter()
        .map(|n| line.find(n))
        .chain(std::iter::once(numerical_index))
        .filter_map(|x| x)
        .min()
        .unwrap()
}

fn match_index_backward(line: &str) -> usize {
    let numerical_index = line.rfind(|c: char| c.is_ascii_digit());
    NUMBERS
        .into_iter()
        .map(|n| line.rfind(n))
        .chain(std::iter::once(numerical_index))
        .filter_map(|x| x)
        .max()
        .unwrap()
}

fn convert_number(line: &str, index: usize) -> usize {
    let n = line.as_bytes()[index];
    if n >= '0' as u8 && n <= '9' as u8 {
        return (n - '0' as u8) as usize;
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

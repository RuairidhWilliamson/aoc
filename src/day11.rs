use std::collections::HashMap;

pub fn solve_part1(input: &str) -> usize {
    blink_stones_map(input, 25)
}

pub fn solve_part2(input: &str) -> usize {
    blink_stones_map(input, 75)
}

fn parse_stones(input: &str) -> impl Iterator<Item = usize> + use<'_> {
    input.split(' ').map(|s| s.parse().unwrap())
}

#[allow(dead_code)]
fn blink_stones_vec(input: &str, n: usize) -> usize {
    let mut stones: Vec<usize> = parse_stones(input.trim()).collect();
    for _ in 0..n {
        let initial_length = stones.len();
        for i in 0..initial_length {
            let s = stones[i];
            match s {
                0 => {
                    stones[i] = 1;
                }
                s if (s.ilog10() + 1) % 2 == 0 => {
                    let middle = 10_usize.pow((s.ilog10() + 1) / 2);
                    let left = s / middle;
                    let right = s % middle;
                    stones[i] = left;
                    stones.push(right);
                }
                s => {
                    stones[i] = 2024 * s;
                }
            }
        }
    }
    stones.len()
}

fn blink_stones_map(input: &str, n: usize) -> usize {
    let mut stones: HashMap<usize, usize> = parse_stones(input.trim()).map(|n| (n, 1)).collect();
    let mut new_stones: HashMap<usize, usize> = HashMap::new();
    for _ in 0..n {
        for (s, c) in &stones {
            match s {
                0 => {
                    *new_stones.entry(1).or_default() += c;
                }
                s if (s.ilog10() + 1) % 2 == 0 => {
                    let middle = 10_usize.pow((s.ilog10() + 1) / 2);
                    let left = s / middle;
                    let right = s % middle;
                    *new_stones.entry(left).or_default() += c;
                    *new_stones.entry(right).or_default() += c;
                }
                s => {
                    *new_stones.entry(2024 * s).or_default() += c;
                }
            }
        }
        std::mem::swap(&mut stones, &mut new_stones);
        new_stones.clear();
    }
    stones.values().sum()
}

#[cfg(test)]
const INPUT: &str = "125 17";

#[test]
fn practice_part1() {
    assert_eq!(blink_stones_vec(INPUT, 25), 55312);
    assert_eq!(blink_stones_map(INPUT, 25), 55312);
}

pub fn part1(input: &str) -> u32 {
    let mut dial: i32 = 50;
    let mut zero_count = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let val = line[1..].parse::<u32>().unwrap() as i32;
        let val = match &line[0..1] {
            "L" => -val,
            "R" => val,
            _ => unreachable!(),
        };
        dial = (dial + val + 100) % 100;
        if dial == 0 {
            zero_count += 1;
        }
    }
    zero_count
}

pub fn part2(input: &str) -> i32 {
    let mut dial: i32 = 50;
    let mut zero_count = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let val = line[1..].parse::<u32>().unwrap() as i32;
        let signed_val = match &line[0..1] {
            "L" => -val,
            "R" => val,
            _ => unreachable!(),
        };
        let complete_spins = val / 100;
        zero_count += complete_spins;
        let old_dial = dial;
        dial += signed_val % 100;
        if old_dial != 0 && (dial <= 0 || dial >= 100) {
            zero_count += 1;
        }
        dial = (dial + 100) % 100;
    }
    zero_count
}

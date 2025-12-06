pub fn part1(input: &str) -> usize {
    inner::<2>(input)
}

pub fn part2(input: &str) -> usize {
    inner::<12>(input)
}

fn inner<const N: usize>(input: &str) -> usize {
    let mut total = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let mut nums = line;
        let mut joltage = 0;
        for i in 0..N {
            let (a_index, &a_byte) = nums.as_bytes()[..nums.len() - (N - i - 1)]
                .iter()
                .enumerate()
                .rev()
                .max_by_key(|(_, x)| *x)
                .unwrap();
            let a = (a_byte as char).to_digit(10).unwrap();
            joltage = joltage * 10 + a as usize;
            nums = &nums[a_index + 1..];
        }
        total += joltage;
    }
    total
}

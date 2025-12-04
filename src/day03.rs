pub fn part1(input: &str) -> u64 {
    inner::<2>(input)
}

pub fn part2(input: &str) -> u64 {
    inner::<12>(input)
}

fn inner<const N: usize>(input: &str) -> u64 {
    let mut total = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let mut nums = &line[..];
        let mut joltage = 0;
        for i in 0..N {
            let (a_index, &a_byte) = nums[..nums.len() - (N - i - 1)]
                .as_bytes()
                .into_iter()
                .enumerate()
                .rev()
                .max_by_key(|(_, x)| *x)
                .unwrap();
            let a = (a_byte as char).to_digit(10).unwrap();
            joltage = joltage * 10 + a as u64;
            nums = &nums[a_index + 1..];
        }
        total += joltage;
    }
    total
}

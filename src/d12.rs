use std::num::NonZeroUsize;

use crate::PartFn;

pub const PARTS: (PartFn, PartFn) = (part1, part2);

fn part1(input: &str) -> isize {
    let total = sum_lines(input, 1);
    total as isize
}

fn part2(input: &str) -> isize {
    let total = sum_lines(input, 5);
    total as isize
}

fn sum_lines(input: &str, repeat: usize) -> usize {
    input
        .lines()
        .map(|l| {
            println!("{l}");
            possible_arrangements(l, repeat)
        })
        .sum()
}

pub fn possible_arrangements(line: &str, repeat: usize) -> usize {
    // println!("{line}");
    let (arr, nums) = line.split_once(' ').unwrap();
    let s: Vec<Spring> = arr.chars().map(|c| c.try_into().unwrap()).collect();
    let dmg_nums: Vec<usize> = nums.split(',').map(|x| x.parse().unwrap()).collect();
    let mut springs = Vec::with_capacity(s.len() * repeat + repeat - 1);
    springs.extend_from_slice(&s);
    for _ in 1..repeat {
        springs.push(Spring::Unknown);
        springs.extend_from_slice(&s);
    }
    let dmg_nums = dmg_nums.repeat(repeat);
    let mut cache = lru::LruCache::new(NonZeroUsize::new(4096).unwrap());

    divide_process(&springs, &dmg_nums, &mut cache)
}

fn divide_process<'a>(
    springs: &'a [Spring],
    dmg_nums: &'a [usize],
    cache: &mut lru::LruCache<Analyzer<'a>, usize>,
) -> usize {
    if springs.is_empty() && dmg_nums.is_empty() {
        return 1;
    } else if springs.is_empty() && !dmg_nums.is_empty() {
        return 0;
    }
    // dbg!(springs, dmg_nums);
    let (a, b) = split_at_first_operational(springs);
    (0..=dmg_nums.len())
        .map(|i| {
            let a_count = Analyzer::start(a, &dmg_nums[..i], cache);
            // Lazy multiplication
            if a_count > 0 {
                a_count * divide_process(b, &dmg_nums[i..], cache)
            } else {
                0
            }
        })
        .sum()
}

fn split_at_first_operational(springs: &[Spring]) -> (&[Spring], &[Spring]) {
    for i in 0..springs.len() {
        if springs[i] == Spring::Operational {
            return (&springs[0..i], &springs[i + 1..]);
        }
    }
    (springs, &[])
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Analyzer<'a> {
    springs: &'a [Spring],
    c: Option<usize>,
    dmg_count: usize,
    dmg_nums: &'a [usize],
    dmg_nums_sum: usize,
}

impl<'a> Analyzer<'a> {
    fn start(
        springs: &'a [Spring],
        dmg_nums: &'a [usize],
        cache: &mut lru::LruCache<Analyzer<'a>, usize>,
    ) -> usize {
        let dmg_count = springs.iter().filter(|&s| s == &Spring::Damaged).count();
        let dmg_nums_sum = dmg_nums.iter().sum::<usize>();
        Self {
            springs,
            c: None,
            dmg_count,
            dmg_nums,
            dmg_nums_sum,
        }
        .analyze(cache)
    }

    fn analyze(self, cache: &mut lru::LruCache<Analyzer<'a>, usize>) -> usize {
        if self.springs.is_empty() {
            return if (self.c == Some(0) || self.c.is_none()) && self.dmg_nums.is_empty() {
                1
            } else {
                0
            };
        }
        if self.dmg_nums.is_empty() && self.c.is_none() {
            return if self
                .springs
                .iter()
                .all(|s| s == &Spring::Operational || s == &Spring::Unknown)
            {
                1
            } else {
                0
            };
        }
        if self.springs.len() < self.dmg_nums_sum {
            return 0;
        }
        if self.dmg_count > self.dmg_nums_sum + self.c.unwrap_or(0) {
            return 0;
        }
        if let Some(cached) = cache.get(&self) {
            return *cached;
        }
        let s = &self.springs[0];
        // println!("{s:?}, {c:?}");
        let result = Self {
            springs: &self.springs[1..],
            c: self.c,
            dmg_count: self.dmg_count,
            dmg_nums: self.dmg_nums,
            dmg_nums_sum: self.dmg_nums_sum,
        }
        .analyze_inner(s, cache);
        cache.put(self, result);
        result
    }

    fn analyze_inner(
        mut self,
        s: &Spring,
        cache: &mut lru::LruCache<Analyzer<'a>, usize>,
    ) -> usize {
        match (s, self.c) {
            (Spring::Operational, None) => self.analyze(cache),
            (Spring::Operational, Some(0)) => {
                self.c = None;
                self.analyze(cache)
            }
            (Spring::Operational, Some(_)) => 0,
            (Spring::Damaged, None) => {
                self.dmg_nums_sum -= self.dmg_nums[0];
                self.c = Some(self.dmg_nums[0] - 1);
                self.dmg_count -= 1;
                self.dmg_nums = &self.dmg_nums[1..];
                self.analyze(cache)
            }
            (Spring::Damaged, Some(0)) => 0,
            (Spring::Damaged, Some(n)) => {
                self.c = Some(n - 1);
                self.dmg_count -= 1;
                self.analyze(cache)
            }
            (Spring::Unknown, _) => {
                let a = self.clone().analyze_inner(&Spring::Operational, cache);
                self.dmg_count += 1;
                a + self.analyze_inner(&Spring::Damaged, cache)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl TryFrom<char> for Spring {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Spring::Operational),
            '#' => Ok(Spring::Damaged),
            '?' => Ok(Spring::Unknown),
            _ => Err(()),
        }
    }
}

#[test]
fn very_simple() {
    assert_eq!(possible_arrangements(".??.# 1,1", 1), 2);
}

#[test]
fn simple_line_example() {
    assert_eq!(possible_arrangements("..### 3", 1), 1);
    assert_eq!(possible_arrangements("..## 3", 1), 0);
    assert_eq!(possible_arrangements("..# 1", 1), 1);
    assert_eq!(possible_arrangements(".#.# 1,1", 1), 1);
    assert_eq!(possible_arrangements(".##.# 1,1", 1), 0);
    assert_eq!(possible_arrangements(".?#.# 1,1", 1), 1);
    assert_eq!(possible_arrangements(".??.# 1,1", 1), 2);
    assert_eq!(possible_arrangements("???.# 1,1", 1), 3);
    assert_eq!(possible_arrangements("???.### 1,1,3", 1), 1);
    assert_eq!(possible_arrangements(".??..??...?##. 1,1,3", 1), 4);
    assert_eq!(possible_arrangements("?#?#?#?#?#?#?#? 1,3,1,6", 1), 1);
    assert_eq!(possible_arrangements("?###???????? 3,2,1", 1), 10);
}

#[test]
fn example() {
    let input = "
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
        "
    .trim()
    .trim_matches('\n');
    let total = sum_lines(input, 1);
    assert_eq!(total, 21);
}

#[test]
fn line_5_repeats() {
    // assert_eq!(possible_arrangements("???.### 1,1,3", 5), 1);
    assert_eq!(possible_arrangements("?###???????? 3,2,1", 5), 506250);
}

#[test]
fn example_5_repeats() {
    let input = "
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
        "
    .trim()
    .trim_matches('\n');
    let total = sum_lines(input, 5);
    assert_eq!(total, 525152);
}

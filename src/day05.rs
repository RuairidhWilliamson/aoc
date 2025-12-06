use std::ops::RangeInclusive;

pub fn part1(input: &str) -> usize {
    let (fresh, ingredients) = input.split_once("\n\n").unwrap();
    let fresh: Vec<RangeInclusive<usize>> = fresh
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            start.parse().unwrap()..=end.parse().unwrap()
        })
        .collect();
    ingredients
        .lines()
        .filter(|line| {
            let ingredient: usize = line.parse().unwrap();
            fresh.iter().any(|f| f.contains(&ingredient))
        })
        .count()
}

pub fn part2(input: &str) -> usize {
    let (fresh_lines, _) = input.split_once("\n\n").unwrap();
    let iter = fresh_lines.lines().map(|line| {
        let (start, end) = line.split_once('-').unwrap();
        let (start, end): (usize, usize) = (start.parse().unwrap(), end.parse().unwrap());
        Range { start, end }
    });
    let mut ranges = Vec::<Range>::new();
    for mut new_rng in iter {
        loop {
            let find_overlap = ranges
                .iter()
                .enumerate()
                .find(|(_, rng)| new_rng.mergeable(**rng));
            if let Some((index, _)) = find_overlap {
                let merging_rng = ranges.swap_remove(index);
                new_rng = new_rng.merge(merging_rng);
            } else {
                break;
            }
        }
        ranges.push(new_rng);
    }
    ranges.into_iter().map(|rng| rng.len()).sum()
}

#[derive(Clone, Copy)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn len(&self) -> usize {
        self.end - self.start + 1
    }

    fn mergeable(self, other: Self) -> bool {
        self.start <= other.end + 1 && other.start <= self.end + 1
    }

    fn merge(self, other: Self) -> Self {
        assert!(self.mergeable(other));
        Self {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}

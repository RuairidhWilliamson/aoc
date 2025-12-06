pub fn part1(input: &str) -> usize {
    let (fresh, ingredients) = input.split_once("\n\n").unwrap();
    let iter = parse_ranges(fresh);
    let mut fresh: Vec<Range> = merge_overlaps(iter);
    fresh.sort_by_key(|Range { start, end: _ }| *start);
    ingredients
        .lines()
        .filter(|line| {
            let ingredient: usize = line.parse().unwrap();
            match fresh.binary_search_by_key(&ingredient, |Range { start, end: _ }| *start) {
                Ok(index) => fresh[index].contains(ingredient),
                Err(0) => false,
                Err(index) => fresh[index - 1].contains(ingredient),
            }
        })
        .count()
}

pub fn part2(input: &str) -> usize {
    let (fresh_lines, _) = input.split_once("\n\n").unwrap();
    let iter = parse_ranges(fresh_lines);
    merge_overlaps(iter).into_iter().map(|rng| rng.len()).sum()
}

fn parse_ranges(input: &str) -> impl Iterator<Item = Range> {
    input.lines().map(|line| {
        let (start, end) = line.split_once('-').unwrap();
        let start = start.parse().unwrap();
        let end = end.parse().unwrap();
        Range { start, end }
    })
}

fn merge_overlaps(iter: impl Iterator<Item = Range>) -> Vec<Range> {
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
    ranges
}

#[derive(Clone, Copy, PartialEq, Eq)]
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

    fn contains(&self, x: usize) -> bool {
        self.start <= x && x <= self.end
    }
}

use std::collections::HashMap;

pub fn solve_part1(input: &str) -> usize {
    let (avaible_patterns, designs) = parse_puzzle(input);
    let mut cache = HashMap::new();
    designs
        .iter()
        .filter(|target| search(&avaible_patterns, target, &mut cache))
        .count()
}

pub fn solve_part2(input: &str) -> usize {
    let (avaible_patterns, designs) = parse_puzzle(input);
    let mut cache = HashMap::new();
    designs
        .iter()
        .map(|target| search_count(&avaible_patterns, target, &mut cache))
        .sum()
}

fn parse_puzzle(input: &str) -> (HashMap<Color, Vec<Pattern>>, Vec<Pattern>) {
    let (available_patterns, designs) = input.split_once("\n\n").unwrap();
    let mut patterns = HashMap::<Color, Vec<Vec<Color>>>::new();
    available_patterns
        .split(", ")
        .map(|pattern| pattern.chars().map(Color::parse).collect::<Vec<_>>())
        .for_each(|p| patterns.entry(p[0]).or_default().push(p));
    let designs = designs
        .lines()
        .map(|l| l.chars().map(Color::parse).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    (patterns, designs)
}

fn search<'a>(
    availabile_patterns: &HashMap<Color, Vec<Pattern>>,
    target_design: &'a [Color],
    cache: &mut HashMap<&'a [Color], bool>,
) -> bool {
    if target_design.is_empty() {
        return true;
    }
    if let Some(has_solution) = cache.get(target_design) {
        return *has_solution;
    }
    let has_solution = availabile_patterns
        .get(&target_design[0])
        .map(|s| s.as_slice())
        .unwrap_or(&[])
        .iter()
        .any(|p| {
            if let Some(t) = target_design.strip_prefix(&p[..]) {
                search(availabile_patterns, t, cache)
            } else {
                false
            }
        });
    cache.insert(target_design, has_solution);
    has_solution
}

fn search_count<'a>(
    availabile_patterns: &HashMap<Color, Vec<Pattern>>,
    target_design: &'a [Color],
    cache: &mut HashMap<&'a [Color], usize>,
) -> usize {
    if target_design.is_empty() {
        return 1;
    }
    if let Some(count) = cache.get(target_design) {
        return *count;
    }
    let count = availabile_patterns
        .get(&target_design[0])
        .map(|s| s.as_slice())
        .unwrap_or(&[])
        .iter()
        .filter_map(|p| {
            Some(search_count(
                availabile_patterns,
                target_design.strip_prefix(&p[..])?,
                cache,
            ))
        })
        .sum();
    cache.insert(target_design, count);
    count
}

type Pattern = Vec<Color>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl Color {
    fn parse(c: char) -> Self {
        match c {
            'w' => Self::White,
            'u' => Self::Blue,
            'b' => Self::Black,
            'r' => Self::Red,
            'g' => Self::Green,
            _ => panic!("unexpected character {c}"),
        }
    }
}

#[cfg(test)]
const INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

#[test]
fn practice_part1() {
    assert_eq!(solve_part1(INPUT), 6);
}

#[test]
fn practice_part2() {
    assert_eq!(solve_part2(INPUT), 16);
}

use std::collections::{HashMap, HashSet};

use aoc_helper::grid::{Grid, GridCell, Vec2};

pub fn solve_part1(input: &str) -> usize {
    let grid: Grid<Cell> = input.parse().unwrap();
    let mut freqs: HashMap<Frequency, Vec<Vec2>> = HashMap::default();
    grid.coords_iter().for_each(|c| {
        let cell = grid.get(c).unwrap();
        match cell {
            Cell::Empty => (),
            Cell::Antenna(freq) => {
                freqs.entry(*freq).or_default().push(c);
            }
        }
    });
    let antinodes: HashSet<Vec2> = freqs
        .values()
        .flat_map(|v| find_all_antinodes(v))
        .filter(|c| grid.get(*c).is_some())
        .collect();
    antinodes.len()
}

pub fn solve_part2(input: &str) -> usize {
    let grid: Grid<Cell> = input.parse().unwrap();
    let mut freqs: HashMap<Frequency, Vec<Vec2>> = HashMap::default();
    grid.coords_iter().for_each(|c| {
        let cell = grid.get(c).unwrap();
        match cell {
            Cell::Empty => (),
            Cell::Antenna(freq) => {
                freqs.entry(*freq).or_default().push(c);
            }
        }
    });
    let antinodes: HashSet<Vec2> = freqs
        .values()
        .flat_map(|v| find_all_antinodes2(&grid, v))
        .collect();
    antinodes.len()
}

fn find_all_antinodes(antennas: &[Vec2]) -> impl Iterator<Item = Vec2> + use<'_> {
    (1..antennas.len()).flat_map(move |i| {
        antennas[i..]
            .iter()
            .flat_map(move |a| find_antinodes(antennas[i - 1], *a))
    })
}

fn find_antinodes(a: Vec2, b: Vec2) -> [Vec2; 2] {
    debug_assert_ne!(a, b);
    let delta = a - b;
    [a + delta, b - delta]
}

fn find_all_antinodes2<'a>(
    grid: &'a Grid<Cell>,
    antennas: &'a [Vec2],
) -> impl Iterator<Item = Vec2> + use<'a> {
    (1..antennas.len()).flat_map(move |i| {
        antennas[i..]
            .iter()
            .flat_map(move |a| find_antinodes2(grid, antennas[i - 1], *a))
    })
}

fn find_antinodes2(grid: &Grid<Cell>, a: Vec2, b: Vec2) -> impl Iterator<Item = Vec2> + use<'_> {
    debug_assert_ne!(a, b);
    let delta = a - b;
    let a_iter = (0..)
        .map(move |i| a + delta * i)
        .take_while(|c| grid.get(*c).is_some());
    let b_iter = (0..)
        .map(move |i| b - delta * i)
        .take_while(|c| grid.get(*c).is_some());
    a_iter.chain(b_iter)
}

enum Cell {
    Empty,
    Antenna(Frequency),
}

impl GridCell for Cell {
    type Err = &'static str;

    fn char_to_cell(c: char) -> Result<Self, Self::Err> {
        match c {
            '.' => Ok(Self::Empty),
            'a'..='z' | 'A'..='Z' | '0'..='9' => Ok(Self::Antenna(Frequency(c))),
            _ => Err("unexpected character"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Frequency(char);

#[cfg(test)]
const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

#[test]
fn practice_part1() {
    assert_eq!(solve_part1(INPUT), 14);
}

#[test]
fn practice_part2() {
    assert_eq!(solve_part2(INPUT), 34);
}

use std::str::FromStr;

use crate::common::grid::{Grid, GridParseError};

pub fn run(input: &str) {
    let total = run_inner(input);
    println!("Total = {total}");
}

fn run_inner(input: &str) -> isize {
    input
        .split("\n\n")
        .map(|pat| pat.parse::<Pattern>().unwrap().find_mirror())
        .sum()
}

struct Pattern {
    grid: Grid<Cell>,
}

const SMUDGES: usize = 1;

impl Pattern {
    fn find_mirror(&self) -> isize {
        for i in 1..self.grid.width() {
            if self.is_vertical_mirror(i) {
                // println!("vertical {i}");
                return i;
            }
        }
        for i in 1..self.grid.height() {
            if self.is_horizontal_mirror(i) {
                // println!("horizontal {i}");
                return i * 100;
            }
        }
        panic!("no mirror found")
    }

    fn is_vertical_mirror(&self, index: isize) -> bool {
        let mut error_count = 0;
        for (x, y) in self.grid.enumerate_coords() {
            let other_x = 2 * index - 1 - x;
            let Some(b) = self.grid.get((other_x, y)) else {
                continue;
            };
            let a = self.grid.get((x, y)).unwrap();
            if a != b {
                error_count += 1;
                if error_count > SMUDGES * 2 {
                    return false;
                }
            }
        }
        error_count == SMUDGES * 2
    }

    fn is_horizontal_mirror(&self, index: isize) -> bool {
        let mut error_count = 0;
        for (x, y) in self.grid.enumerate_coords() {
            let other_y = 2 * index - 1 - y;
            let Some(b) = self.grid.get((x, other_y)) else {
                continue;
            };
            let a = self.grid.get((x, y)).unwrap();
            if a != b {
                error_count += 1;
                if error_count > SMUDGES * 2 {
                    return false;
                }
            }
        }
        error_count == SMUDGES * 2
    }
}

impl FromStr for Pattern {
    type Err = GridParseError<MyError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s.parse()?;
        Ok(Self { grid })
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Cell {
    Ash,
    Rock,
}

impl TryFrom<char> for Cell {
    type Error = MyError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Cell::Ash),
            '#' => Ok(Cell::Rock),
            c => Err(MyError::UnknownCellChar(c)),
        }
    }
}

#[derive(Debug)]
enum MyError {
    UnknownCellChar(char),
}

#[test]
fn example() {
    let input = "
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.
    "
    .trim()
    .trim_matches('\n');
    let pat: Pattern = input.parse().unwrap();
    assert_eq!(pat.find_mirror(), 300);
}

#[test]
fn example2() {
    let input = "
#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
        "
    .trim()
    .trim_matches('\n');
    let pat: Pattern = input.parse().unwrap();
    assert_eq!(pat.find_mirror(), 100);
}

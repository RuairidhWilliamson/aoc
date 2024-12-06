use aoc_helper::grid::{Grid, Vec2};

pub fn solve_part1(input: &str) -> usize {
    let grid: Grid<char> = input.parse().unwrap();
    grid.coords_iter()
        .map(|c| {
            if grid.get(c).unwrap() == &'X' {
                [
                    check_line(&grid, c, Vec2::new(1, 0)),
                    check_line(&grid, c, Vec2::new(-1, 0)),
                    check_line(&grid, c, Vec2::new(0, 1)),
                    check_line(&grid, c, Vec2::new(0, -1)),
                    check_line(&grid, c, Vec2::new(1, 1)),
                    check_line(&grid, c, Vec2::new(1, -1)),
                    check_line(&grid, c, Vec2::new(-1, 1)),
                    check_line(&grid, c, Vec2::new(-1, -1)),
                ]
                .iter()
                .flatten()
                .count()
            } else {
                0
            }
        })
        .sum()
}

fn check_line(grid: &Grid<char>, start: Vec2, direction: Vec2) -> Option<()> {
    for (i, c) in "XMAS".chars().enumerate() {
        let point = start + direction * i as isize;
        let cell = grid.get(point)?;
        if cell != &c {
            return None;
        }
    }
    Some(())
}

pub fn solve_part2(input: &str) -> usize {
    let grid: Grid<char> = input.parse().unwrap();
    grid.coords_iter()
        .map(|c| {
            if grid.get(c).unwrap() == &'A' {
                [
                    check_x_mas(&grid, c, Direction::Up),
                    check_x_mas(&grid, c, Direction::Down),
                    check_x_mas(&grid, c, Direction::Left),
                    check_x_mas(&grid, c, Direction::Right),
                ]
                .iter()
                .flatten()
                .count()
            } else {
                0
            }
        })
        .sum()
}

#[repr(usize)]
#[derive(Debug, Clone, Copy)]
enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl Direction {
    fn mas(self) -> impl Iterator<Item = (Vec2, char)> {
        let mut expects = ['M', 'M', 'S', 'S'];
        expects.rotate_right(self as usize);
        [(1, 1), (-1, 1), (-1, -1), (1, -1)]
            .map(|v| Vec2::try_from(v).unwrap())
            .into_iter()
            .zip(expects)
    }
}

fn check_x_mas(grid: &Grid<char>, start: Vec2, direction: Direction) -> Option<()> {
    let center = grid.get(start)?;
    if center != &'A' {
        return None;
    }
    for (d, expected) in direction.mas() {
        let n = start + d;
        if grid.get(n)? != &expected {
            return None;
        }
    }
    Some(())
}

#[cfg(test)]
const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

#[test]
fn practice_part1() {
    assert_eq!(solve_part1(INPUT), 18);
}

#[test]
fn practice_part2() {
    assert_eq!(solve_part2(INPUT), 9);
}

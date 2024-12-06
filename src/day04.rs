use crate::grid::Grid;

pub fn solve_part1(input: &str) -> usize {
    let grid: Grid<char> = input.parse().unwrap();
    grid.coords_iter()
        .map(|c| {
            if grid.get_old(c).unwrap() == &'X' {
                [
                    check_line(&grid, c, (1, 0)),
                    check_line(&grid, c, (-1, 0)),
                    check_line(&grid, c, (0, 1)),
                    check_line(&grid, c, (0, -1)),
                    check_line(&grid, c, (1, 1)),
                    check_line(&grid, c, (1, -1)),
                    check_line(&grid, c, (-1, 1)),
                    check_line(&grid, c, (-1, -1)),
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

fn check_line(
    grid: &Grid<char>,
    (start_x, start_y): (usize, usize),
    (direction_x, direction_y): (isize, isize),
) -> Option<()> {
    for (i, c) in "XMAS".chars().enumerate() {
        let x = start_x.checked_add_signed(direction_x * i as isize)?;
        let y = start_y.checked_add_signed(direction_y * i as isize)?;
        let cell = grid.get_old((x, y))?;
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
            if grid.get_old(c).unwrap() == &'A' {
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
    fn mas(self) -> impl Iterator<Item = ((isize, isize), char)> {
        let mut expects = ['M', 'M', 'S', 'S'];
        expects.rotate_right(self as usize);
        [(1, 1), (-1, 1), (-1, -1), (1, -1)]
            .into_iter()
            .zip(expects)
    }
}

fn check_x_mas(
    grid: &Grid<char>,
    (start_x, start_y): (usize, usize),
    direction: Direction,
) -> Option<()> {
    let center = grid.get_old((start_x, start_y))?;
    if center != &'A' {
        return None;
    }
    for ((d_x, d_y), expected) in direction.mas() {
        let x = start_x.checked_add_signed(d_x)?;
        let y = start_y.checked_add_signed(d_y)?;
        if grid.get_old((x, y))? != &expected {
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

use std::io::stdin;

use aoc::grid::Grid;

fn main() {
    let input = std::io::read_to_string(stdin()).unwrap();
    let grid = input.parse().unwrap();
    let total = total_dist(&grid, 1000000);
    println!("{total}");
}

fn expanded_rows(grid: &Grid<MapCell>) -> Vec<isize> {
    (0..grid.height())
        .filter(|y| {
            (0..grid.width()).all(|x| {
                let c = (x, *y);
                grid.get(c).unwrap() == &MapCell::Space
            })
        })
        .collect()
}

fn expanded_columns(grid: &Grid<MapCell>) -> Vec<isize> {
    (0..grid.width())
        .filter(|x| {
            (0..grid.height()).all(|y| {
                let c = (*x, y);
                grid.get(c).unwrap() == &MapCell::Space
            })
        })
        .collect()
}

fn total_dist(grid: &Grid<MapCell>, expand: usize) -> usize {
    let expand = expand - 1;
    let rows = expanded_rows(grid);
    let columns = expanded_columns(grid);
    let galaxies: Vec<_> = grid
        .enumerate_coords()
        .filter(|c| grid.get(*c).unwrap() == &MapCell::Galaxy)
        .collect();
    let mut total = 0;
    for a in &galaxies {
        for b in &galaxies {
            if a == b {
                break;
            }
            total += a.0.abs_diff(b.0)
                + a.1.abs_diff(b.1)
                + rows.iter().filter(|&y| (a.1 < *y) != (b.1 < *y)).count() * expand
                + columns.iter().filter(|&x| (a.0 < *x) != (b.0 < *x)).count() * expand;
        }
    }
    total
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum MapCell {
    Galaxy,
    Space,
}

impl TryFrom<char> for MapCell {
    type Error = MyError;

    fn try_from(v: char) -> Result<Self, Self::Error> {
        match v {
            '#' => Ok(Self::Galaxy),
            '.' => Ok(Self::Space),
            c => Err(MyError::UnknownChar(c)),
        }
    }
}

impl From<&MapCell> for char {
    fn from(value: &MapCell) -> Self {
        match value {
            MapCell::Galaxy => '#',
            MapCell::Space => '.',
        }
    }
}

#[derive(Debug, aoc::thiserror::Error)]
enum MyError {
    #[error("unknown char {0:?}")]
    UnknownChar(char),
}

#[test]
fn simple_example() {
    let input = "
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
    "
    .trim()
    .trim_matches('\n');

    let grid: Grid<MapCell> = input.parse().unwrap();
    println!("{grid}");

    let d = total_dist(&grid, 2);
    assert_eq!(d, 374);

    let d = total_dist(&grid, 10);
    assert_eq!(d, 1030);

    let d = total_dist(&grid, 100);
    assert_eq!(d, 8410);
}

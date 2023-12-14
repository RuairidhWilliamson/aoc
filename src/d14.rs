use kdam::tqdm;

use crate::common::grid::Grid;

pub fn run(input: &str) {
    let output = run_inner(input, 1000000000);
    println!("{output}");
}

fn run_inner(input: &str, cycles: usize) -> usize {
    let mut grid: Grid<Cell> = input.parse().unwrap();
    slide_grid(&mut grid, cycles);

    // println!("{grid}");
    // Calculate north beam load
    grid.enumerate_coords()
        .map(|c| {
            let cell = grid.get(c).unwrap();
            match cell {
                Cell::RoundRock => grid.height() - c.1,
                _ => 0,
            }
        })
        .sum::<isize>() as usize
}

fn slide_grid(grid: &mut Grid<Cell>, cycles: usize) {
    let mut last_multiple_index = 0;
    let mut n: usize = 10;
    let mut last_multiple_of_four_grid: Option<Grid<Cell>> = None;
    for i in tqdm!(0..cycles) {
        if last_multiple_of_four_grid.as_ref() == Some(grid) {
            println!("Cycle found");
            let period = i - last_multiple_index;
            let more_cycles = (cycles - i) % period;
            slide_grid(grid, more_cycles);
            break;
        }
        if i % n == 0 {
            last_multiple_index = i;
            last_multiple_of_four_grid = Some(grid.to_owned());
            n = i.min(cycles / 1000).max(10);
        }
        slide_grid_north(grid);
        slide_grid_west(grid);
        slide_grid_south(grid);
        slide_grid_east(grid);
    }
}
fn slide_grid_north(grid: &mut Grid<Cell>) {
    for x in 0..grid.width() {
        let mut square_y = 0;
        for y in 0..grid.height() {
            match grid.get((x, y)).unwrap() {
                Cell::RoundRock => {
                    let c = grid.get_mut((x, y)).unwrap();
                    *c = Cell::Empty;
                    let c = grid.get_mut((x, square_y)).unwrap();
                    *c = Cell::RoundRock;
                    square_y += 1;
                }
                Cell::SquareRock => {
                    square_y = y + 1;
                }
                Cell::Empty => (),
            }
        }
    }
}

fn slide_grid_west(grid: &mut Grid<Cell>) {
    for y in 0..grid.height() {
        let mut square_x = 0;
        for x in 0..grid.width() {
            match grid.get((x, y)).unwrap() {
                Cell::RoundRock => {
                    let c = grid.get_mut((x, y)).unwrap();
                    *c = Cell::Empty;
                    let c = grid.get_mut((square_x, y)).unwrap();
                    *c = Cell::RoundRock;
                    square_x += 1;
                }
                Cell::SquareRock => {
                    square_x = x + 1;
                }
                Cell::Empty => (),
            }
        }
    }
}

fn slide_grid_south(grid: &mut Grid<Cell>) {
    for x in 0..grid.width() {
        let mut square_y = grid.height() - 1;
        for y in (0..grid.height()).rev() {
            match grid.get((x, y)).unwrap() {
                Cell::RoundRock => {
                    let c = grid.get_mut((x, y)).unwrap();
                    *c = Cell::Empty;
                    let c = grid.get_mut((x, square_y)).unwrap();
                    *c = Cell::RoundRock;
                    square_y -= 1;
                }
                Cell::SquareRock => {
                    square_y = y - 1;
                }
                Cell::Empty => (),
            }
        }
    }
}

fn slide_grid_east(grid: &mut Grid<Cell>) {
    for y in 0..grid.height() {
        let mut square_x = grid.width() - 1;
        for x in (0..grid.width()).rev() {
            match grid.get((x, y)).unwrap() {
                Cell::RoundRock => {
                    let c = grid.get_mut((x, y)).unwrap();
                    *c = Cell::Empty;
                    let c = grid.get_mut((square_x, y)).unwrap();
                    *c = Cell::RoundRock;
                    square_x -= 1;
                }
                Cell::SquareRock => {
                    square_x = x - 1;
                }
                Cell::Empty => (),
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cell {
    RoundRock,
    SquareRock,
    Empty,
}

impl TryFrom<char> for Cell {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'O' => Ok(Self::RoundRock),
            '#' => Ok(Self::SquareRock),
            '.' => Ok(Self::Empty),
            _ => Err(()),
        }
    }
}

impl From<&Cell> for char {
    fn from(value: &Cell) -> Self {
        match value {
            Cell::RoundRock => 'O',
            Cell::SquareRock => '#',
            Cell::Empty => '.',
        }
    }
}

#[test]
fn example1() {
    let mut grid: Grid<Cell> = "
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
    "
    .trim()
    .trim_matches('\n')
    .parse()
    .unwrap();

    let expected_north: Grid<Cell> = "
OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....
    "
    .trim()
    .trim_matches('\n')
    .parse()
    .unwrap();
    slide_grid_north(&mut grid);
    println!("{grid}");
    println!("{expected_north}");
    assert_eq!(expected_north, grid);
}

#[test]
fn example2() {
    let input = "
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
    "
    .trim()
    .trim_matches('\n');
    let expected = "
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O
    "
    .trim()
    .trim_matches('\n');
    let mut grid: Grid<Cell> = input.parse().unwrap();
    let expected_grid: Grid<Cell> = expected.parse().unwrap();
    slide_grid(&mut grid, 3);
    println!("{grid}");
    println!("{expected_grid}");
    assert_eq!(expected_grid, grid);
}

#[test]
fn example3() {
    let input = "
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
    "
    .trim()
    .trim_matches('\n');
    assert_eq!(run_inner(input, 1000000000), 64);
}

#[test]
fn example4() {
    let input = "
..........
........#.
..........
..........
..........
..........
..........
..........
..........
........OO
    "
    .trim()
    .trim_matches('\n');
    assert_eq!(run_inner(input, 1000000000), 2);
}

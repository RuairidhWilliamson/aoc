use crate::{
    common::grid::{add_coords, Coord, Grid},
    PartFn,
};

pub const PARTS: (PartFn, PartFn) = (part1, part2);

fn part1(input: &str) -> usize {
    find_longest_route(&input.parse().unwrap())
}

fn part2(input: &str) -> usize {
    find_longest_route_no_slopes(&input.parse().unwrap())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rel_coord(&self) -> Coord {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Forest,
    Path,
    Slope(Direction),
}

impl TryFrom<char> for Cell {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::Forest),
            '.' => Ok(Self::Path),
            '^' => Ok(Self::Slope(Direction::North)),
            '>' => Ok(Self::Slope(Direction::East)),
            'v' => Ok(Self::Slope(Direction::South)),
            '<' => Ok(Self::Slope(Direction::West)),
            c => Err(c),
        }
    }
}

fn find_longest_route(grid: &Grid<Cell>) -> usize {
    let start_cell = (0..grid.width())
        .map(|x| (x, 0))
        .find(|&c| grid.get(c) != Some(&Cell::Forest))
        .unwrap();
    search_longest(grid, start_cell, vec![]).unwrap()
}

fn search_longest(grid: &Grid<Cell>, mut start: Coord, mut visited: Vec<Coord>) -> Option<usize> {
    loop {
        if start.1 == grid.height() - 1 {
            return Some(visited.len());
        }
        visited.push(start);
        let cell = grid.get(start).unwrap();
        let directions = match *cell {
            Cell::Slope(d) => vec![d],
            Cell::Path => vec![
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ],
            Cell::Forest => panic!("unexpected forest"),
        };
        let new_cs: Vec<_> = directions
            .into_iter()
            .filter_map(|d| {
                let new_c = add_coords(start, d.rel_coord());
                let cell = grid.get(new_c);
                if matches!(cell, Some(Cell::Forest) | None) || visited.contains(&new_c) {
                    None
                } else {
                    Some(new_c)
                }
            })
            .collect();
        if new_cs.is_empty() {
            return None;
        }
        if new_cs.len() == 1 {
            start = new_cs[0];
        } else {
            return new_cs
                .iter()
                .filter_map(|&new_c| search_longest(grid, new_c, visited.clone()))
                .max();
        }
    }
}

fn find_longest_route_no_slopes(grid: &Grid<Cell>) -> usize {
    let start_cell = (0..grid.width())
        .map(|x| (x, 0))
        .find(|&c| grid.get(c) != Some(&Cell::Forest))
        .unwrap();
    search_longest_no_slopes(grid, start_cell, (-100, -100), vec![], 0).unwrap()
}

fn search_longest_no_slopes(
    grid: &Grid<Cell>,
    mut start: Coord,
    mut previous: Coord,
    mut visited: Vec<Coord>,
    mut length: usize,
) -> Option<usize> {
    loop {
        if start.1 == grid.height() - 1 {
            return Some(length);
        }
        let directions = [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ];
        let new_cs: Vec<_> = directions
            .into_iter()
            .filter_map(|d| {
                let new_c = add_coords(start, d.rel_coord());
                // println!("{new_c:?}");
                if previous == new_c {
                    return None;
                }
                let cell = grid.get(new_c);
                if matches!(cell, Some(Cell::Forest) | None) || visited.contains(&new_c) {
                    None
                } else {
                    Some(new_c)
                }
            })
            .collect();
        if new_cs.is_empty() {
            return None;
        }
        if new_cs.len() == 1 {
            previous = start;
            start = new_cs[0];
            length += 1;
        } else {
            visited.push(start);
            return new_cs[1..]
                .iter()
                .filter_map(|&new_c| {
                    search_longest_no_slopes(grid, new_c, start, visited.clone(), length + 1)
                })
                .max()
                .max(search_longest_no_slopes(
                    grid,
                    new_cs[0],
                    start,
                    visited,
                    length + 1,
                ));
        }
    }
}

#[test]
fn example1() {
    let input = "
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
    ";

    assert_eq!(
        find_longest_route(&input.trim().trim_matches('\n').parse().unwrap()),
        94
    );
}

#[test]
fn example2() {
    let input = "
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
    ";

    assert_eq!(
        find_longest_route_no_slopes(&input.trim().trim_matches('\n').parse().unwrap()),
        154
    );
}

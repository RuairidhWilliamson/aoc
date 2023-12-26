#![allow(dead_code)]
use std::collections::HashSet;

use rayon::prelude::ParallelIterator;

use crate::{
    common::grid::{add_coords, Coord, Grid},
    PartFn,
};

pub const PARTS: (PartFn, PartFn) = (part1, part2);

fn part1(input: &str) -> usize {
    let mut grid: Grid<Cell> = input.trim().trim_matches('\n').parse().unwrap();
    grid = step_n(grid, 64);
    count_starts(&grid)
}

fn part2(input: &str) -> usize {
    let grid: Grid<Cell> = input.trim().trim_matches('\n').parse().unwrap();
    println!("{} {}", grid.width(), grid.height());
    // count_starts_steps_repeating_n(&grid, 26501365) as isize
    best_method(&grid, 26501365)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Rock,
    Start,
}

impl TryFrom<char> for Cell {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            '#' => Ok(Self::Rock),
            'S' => Ok(Self::Start),
            _ => Err(()),
        }
    }
}

fn step(old_grid: Grid<Cell>) -> Grid<Cell> {
    let mut new_grid = old_grid.clone();
    for c in old_grid.enumerate_coords() {
        let new = new_grid.get_mut(c).unwrap();
        if new != &Cell::Start {
            continue;
        }
        *new = Cell::Empty;
    }
    for c in old_grid.enumerate_coords() {
        let old = old_grid.get(c).unwrap();
        if old != &Cell::Start {
            continue;
        }
        for rel in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let new_c = add_coords(c, rel);
            if let Some(cell) = new_grid.get_mut(new_c) {
                if *cell == Cell::Rock {
                    continue;
                }
                *cell = Cell::Start;
            }
        }
    }
    new_grid
}

fn step_n(mut old_grid: Grid<Cell>, n: usize) -> Grid<Cell> {
    for _ in 0..n {
        old_grid = step(old_grid);
    }
    old_grid
}

fn count_starts(grid: &Grid<Cell>) -> usize {
    grid.enumerate_coords()
        .filter(|&c| grid.get(c).unwrap() == &Cell::Start)
        .count()
}

pub fn count_starts_steps_repeating_n(grid: &Grid<Cell>, n: usize) -> HashSet<Coord> {
    let mut visited: HashSet<Coord> = HashSet::new();
    let mut to_visit = [Vec::new(), Vec::new()];
    let start_coord = grid
        .enumerate_coords()
        .find(|&c| grid.get(c) == Some(&Cell::Start))
        .unwrap();
    to_visit[0].push(start_coord);

    for _ in kdam::tqdm!(0..n) {
        let [rd, wr] = &mut to_visit;
        for c in rd.drain(..) {
            if !visited.insert(c) {
                continue;
            }
            for rel in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
                let new_c = add_coords(c, rel);
                if visited.contains(&new_c) {
                    continue;
                }
                let local_c = coord_mod(new_c, (grid.width(), grid.height()));
                if grid.get(local_c).unwrap() == &Cell::Rock {
                    continue;
                }
                wr.push(new_c);
            }
        }

        to_visit.swap(0, 1);
    }

    to_visit[0].sort_unstable();
    to_visit[0].dedup();
    let start_parity = coord_parity(start_coord);
    let desired_parity = (start_parity + n as isize) % 2;
    visited
        .into_iter()
        .filter(|&c| coord_parity(c) == desired_parity)
        .chain(to_visit[0].iter().copied())
        .collect()
}

fn coord_mod(c: Coord, rhs: Coord) -> Coord {
    (c.0.rem_euclid(rhs.0), c.1.rem_euclid(rhs.1))
}

fn coord_parity(c: Coord) -> isize {
    (c.0.rem_euclid(2) + c.1.rem_euclid(2)) % 2
}

fn estimated_count_method(grid: &Grid<Cell>, n: usize) -> usize {
    let m = 0;
    let grid_size = grid.width().max(grid.height());
    let start_n = n as isize - grid_size * m;
    let density = grid_empty_density(&grid);
    let base_area = start_n as f64 * start_n as f64 * density;
    base_area as usize
}

fn grid_empty_count(grid: &Grid<Cell>) -> usize {
    grid.enumerate_coords()
        .filter(|&c| grid.get(c) != Some(&Cell::Rock))
        .count()
}

fn grid_empty_density(grid: &Grid<Cell>) -> f64 {
    grid_empty_count(grid) as f64 / (grid.width() * grid.height()) as f64
}

// named like this not because it is the best method but because I hoped naming it this would manifest the best method
pub fn best_method(grid: &Grid<Cell>, n: usize) -> usize {
    assert_eq!(grid.width(), grid.height());
    // let mut visited = count_starts_steps_repeating_n(grid, n);
    let grid_size = grid.width();

    let corners = [
        (0, 0),
        (grid_size - 1, 0),
        (grid_size - 1, grid_size - 1),
        (0, grid_size - 1),
    ];
    let start_coord = grid
        .enumerate_coords()
        .find(|&c| grid.get(c) == Some(&Cell::Start))
        .unwrap();
    let start_parity = coord_parity(start_coord);
    let desired_parity = (start_parity + n as isize) % 2;
    let distance_from_start = find_grid_distances(grid, start_coord);
    let distance_from_corners = corners.map(|c| find_grid_distances(grid, c));
    let max_distance_from_corners = distance_from_corners.clone().map(|distance| {
        distance
            .enumerate_coords()
            .filter_map(|c| distance.get(c).unwrap().ok())
            .max()
            .unwrap()
    });
    // println!("{}", distance_from_corners[0]);
    // println!("{distance_from_start}");
    // let [min_corner, max_corner] = min_max_corner(&distance_from_start);
    // let d = n as isize - min_corner;
    // let m = *max_distance_from_corners.iter().max().unwrap();
    let m = 2 * grid_size;
    let d = n as isize - m as isize;
    println!("{}", d);
    let tile_radius = d.div_euclid(grid_size);
    let even_tiles_cell_count = distances_non_inf_count(&distance_from_start, desired_parity);
    let odd_tiles_cell_count = distances_non_inf_count(&distance_from_start, 1 - desired_parity);
    assert!(tile_radius > 0);
    let mut total = 0;
    for extra_radius in 0..8 {
        let acc: usize = diamond(tile_radius + extra_radius)
            .map(|tc| {
                let base_corner = (tc.0 * grid_size, tc.1 * grid_size);
                let corner_distances = corners.map(|c| {
                    calc_corner_distance(add_coords(c, base_corner), &distance_from_start)
                });
                // if (0..4).all(|i| corner_distances[i] + max_distance_from_corners[i] < n) {
                //     return if coord_parity(base_corner) == 0 {
                //         even_tiles_cell_count
                //     } else {
                //         odd_tiles_cell_count
                //     };
                // }
                if extra_radius == 0 && (0..4).any(|i| corner_distances[i] > n) {
                    panic!("first radius too small")
                }
                grid.enumerate_coords()
                    .filter(|&c| {
                        let actual_coord = add_coords(c, base_corner);
                        if coord_parity(actual_coord) != desired_parity {
                            return false;
                        }
                        (0..4).any(|i| {
                            let corner_distance = corner_distances[i];
                            if corner_distance > n {
                                return false;
                            }
                            let max_dist = n - corner_distance;
                            let distance_grid = &distance_from_corners[i];
                            if let Distance::D(d) = distance_grid.get(c).unwrap() {
                                d <= &max_dist
                            } else {
                                false
                            }
                        })
                    })
                    .count()
            })
            .sum();
        println!("acc = {}", acc);
        if acc == 0 {
            break;
        }
        total += acc;
    }
    let even_tiles = count_tiles_in_diamond(tile_radius as usize - 1, desired_parity as usize);
    let odd_tiles = count_tiles_in_diamond(tile_radius as usize - 1, 1 - desired_parity as usize);
    total + even_tiles * even_tiles_cell_count + odd_tiles * odd_tiles_cell_count
}

fn count_tiles_in_diamond(radius: usize, parity: usize) -> usize {
    if radius == 0 {
        return 1 - parity;
    }
    if radius % 2 == parity {
        4 * radius + count_tiles_in_diamond(radius - 1, parity)
    } else {
        0 + count_tiles_in_diamond(radius - 1, parity)
    }
}

fn calc_corner_distance(c: Coord, distance: &Grid<Distance>) -> usize {
    [
        (0, 0),
        (0, distance.height() - 1),
        (distance.width() - 1, 0),
        (distance.width() - 1, distance.height() - 1),
    ]
    .into_iter()
    .map(|corner| {
        let d = corner.0.abs_diff(c.0) + corner.1.abs_diff(c.1);
        d + distance.get(corner).unwrap().unwrap()
    })
    .min()
    .unwrap()
}

fn diamond(n: isize) -> impl rayon::iter::ParallelIterator<Item = Coord> {
    use rayon::prelude::*;
    (-n..=n)
        .into_par_iter()
        .map(move |i| (i, n - i.abs()))
        .chain(
            (1 - n..=n - 1)
                .into_par_iter()
                .map(move |i| (i, i.abs() - n)),
        )
}

fn find_grid_distances(grid: &Grid<Cell>, start_coord: Coord) -> Grid<Distance> {
    let mut distance = Grid::new(Distance::Inf, grid.width() as usize, grid.height() as usize);
    let mut to_visit = [Vec::new(), Vec::new()];
    to_visit[0].push(start_coord);

    let mut i = 0;
    while !to_visit[0].is_empty() {
        let [rd, wr] = &mut to_visit;
        for c in rd.drain(..) {
            let cell = distance.get_mut(c).unwrap();
            if matches!(*cell, Distance::D(_)) {
                continue;
            }
            *cell = Distance::D(i);
            for rel in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
                let new_c = add_coords(c, rel);
                let cell = distance.get(new_c);
                if !matches!(cell, Some(&Distance::Inf)) {
                    continue;
                }
                let cell = grid.get(new_c);
                if cell == Some(&Cell::Rock) || cell == None {
                    continue;
                }
                wr.push(new_c);
            }
        }
        to_visit.swap(0, 1);
        i += 1;
    }
    distance
}

fn distances_non_inf_count(distance: &Grid<Distance>, parity: isize) -> usize {
    distance
        .enumerate_coords()
        .filter(|&c| coord_parity(c) == parity)
        .filter(|&c| matches!(distance.get(c), Some(Distance::D(_))))
        .count()
}

fn min_max_corner(distance: &Grid<Distance>) -> [isize; 2] {
    let d0 = distance.get((0, 0)).unwrap().unwrap() as isize;
    let d1 = distance.get((distance.width() - 1, 0)).unwrap().unwrap() as isize;
    let d2 = distance
        .get((distance.width() - 1, distance.height() - 1))
        .unwrap()
        .unwrap() as isize;
    let d3 = distance.get((0, distance.height() - 1)).unwrap().unwrap() as isize;
    [d0.min(d1).min(d2).min(d3), d0.max(d1).max(d2).max(d3)]
}

#[derive(Debug, Clone)]
enum Distance {
    D(usize),
    Inf,
}

impl Distance {
    fn unwrap(&self) -> usize {
        let &Distance::D(d) = self else {
            panic!("unwrap Distance::Inf");
        };
        d
    }

    fn ok(&self) -> Option<usize> {
        match self {
            Self::D(d) => Some(*d),
            Self::Inf => None,
        }
    }
}

impl From<&Distance> for char {
    fn from(value: &Distance) -> Self {
        match value {
            Distance::D(i) => ((i % 10) as u8 + b'0') as char,
            Distance::Inf => '.',
        }
    }
}

#[test]
fn example1() {
    let input = "
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
    ";
    let mut grid: Grid<Cell> = input.trim().trim_matches('\n').parse().unwrap();
    assert_eq!(count_starts(&grid), 1);
    grid = step_n(grid, 6);
    assert_eq!(count_starts(&grid), 16);
}

/*
#[test]
fn example2() {
    let input = "
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
    ";
    let grid: Grid<Cell> = input.trim().trim_matches('\n').parse().unwrap();
    assert_eq!(count_starts_steps_repeating_n(&grid, 0), 1);
    assert_eq!(count_starts_steps_repeating_n(&grid, 1), 2);
    assert_eq!(count_starts_steps_repeating_n(&grid, 2), 4);
    assert_eq!(count_starts_steps_repeating_n(&grid, 3), 6);
    assert_eq!(count_starts_steps_repeating_n(&grid, 6), 16);
    assert_eq!(count_starts_steps_repeating_n(&grid, 10), 50);
    assert_eq!(count_starts_steps_repeating_n(&grid, 50), 1594);
    assert_eq!(count_starts_steps_repeating_n(&grid, 100), 6536);
    // assert_eq!(count_starts_steps_repeating_n(&grid, 500), 167004);
    // assert_eq!(count_starts_steps_repeating_n(&grid, 1000), 668697);
    // assert_eq!(count_starts_steps_repeating_n(&grid, 5000), 16733044);
}
*/

#[test]
fn example3() {
    let input = "
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
    ";
    let grid: Grid<Cell> = input.trim().trim_matches('\n').parse().unwrap();

    assert_eq!(best_method(&grid, 50), 1594);
    assert_eq!(best_method(&grid, 100), 6536);
    assert_eq!(best_method(&grid, 500), 167004);
    assert_eq!(best_method(&grid, 1000), 668697);
    assert_eq!(best_method(&grid, 5000), 16733044);
}

#[test]
fn test_count_tiles_in_diamond() {
    assert_eq!(count_tiles_in_diamond(1, 0), 1);
    assert_eq!(count_tiles_in_diamond(1, 1), 4);
    assert_eq!(count_tiles_in_diamond(2, 0), 9);
}

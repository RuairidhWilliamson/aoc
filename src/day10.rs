use std::collections::{HashMap, HashSet};

use aoc_helper::grid::{Direction, Grid, GridCell, Vec2};

struct Cell(u8);

impl GridCell for Cell {
    type Err = &'static str;

    fn char_to_cell(c: char) -> Result<Self, Self::Err> {
        if c.is_ascii_digit() {
            Ok(Self(c as u8 - b'0'))
        } else {
            Err("not an ascii digit from 0-9")
        }
    }
}

pub fn solve_part1(input: &str) -> usize {
    let grid: Grid<Cell> = input.parse().unwrap();
    find_trailheads(&grid)
        .map(|head| find_trailtails(&grid, head).len())
        .sum()
}

pub fn solve_part2(input: &str) -> usize {
    let grid: Grid<Cell> = input.parse().unwrap();
    find_trailheads(&grid)
        .map(|head| {
            count_unique_trails(&grid, head)
                .into_values()
                .sum::<usize>()
        })
        .sum()
}

fn find_trailheads(grid: &Grid<Cell>) -> impl Iterator<Item = Vec2> + use<'_> {
    grid.coords_iter().filter(|c| {
        let cell = grid.get(*c).unwrap();
        cell.0 == 0
    })
}

fn find_trailtails(grid: &Grid<Cell>, head: Vec2) -> HashSet<Vec2> {
    let mut heads = HashSet::<Vec2>::new();
    heads.insert(head);
    for i in 1..=9 {
        let mut new_heads = HashSet::<Vec2>::new();
        for h in heads {
            for d in Direction::variants_as_array() {
                let adj_coord = h + Vec2::from(d);
                let Some(adj) = grid.get(adj_coord) else {
                    continue;
                };
                if adj.0 == i {
                    new_heads.insert(adj_coord);
                }
            }
        }
        heads = new_heads;
    }
    heads
}

fn count_unique_trails(grid: &Grid<Cell>, head: Vec2) -> HashMap<Vec2, usize> {
    let mut heads = HashMap::<Vec2, usize>::new();
    heads.insert(head, 1);
    for i in 1..=9 {
        let mut new_heads = HashMap::<Vec2, usize>::new();
        for (h, count) in heads {
            for d in Direction::variants_as_array() {
                let adj_coord = h + Vec2::from(d);
                let Some(adj) = grid.get(adj_coord) else {
                    continue;
                };
                if adj.0 == i {
                    *new_heads.entry(adj_coord).or_default() += count;
                }
            }
        }
        heads = new_heads;
    }
    heads
}

#[cfg(test)]
const INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

#[test]
fn practice_part1() {
    assert_eq!(solve_part1(INPUT), 36);
}

#[test]
fn practice_part2() {
    assert_eq!(solve_part2(INPUT), 81);
}

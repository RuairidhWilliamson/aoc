use std::convert::Infallible;

use aoc_helper::grid::{Direction, Grid, Vec2};

pub fn solve_part1(input: &str) -> usize {
    let mut garden: Garden = Garden::parse(input);
    garden.flood_fill()
}

pub fn solve_part2(input: &str) -> usize {
    let mut garden: Garden = Garden::parse(input);
    garden.flood_fill2()
}

#[derive(Debug)]
struct Cell {
    plot: char,
    visited: bool,
}

impl TryFrom<char> for Cell {
    type Error = Infallible;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(Self {
            plot: c,
            visited: false,
        })
    }
}

struct Garden {
    grid: Grid<Cell>,
}

impl Garden {
    fn parse(input: &str) -> Self {
        Self {
            grid: input.parse().unwrap(),
        }
    }

    fn flood_fill(&mut self) -> usize {
        let mut total = 0;
        let mut to_visit_other: Vec<Vec2> = Vec::new();
        to_visit_other.push(Vec2::new(0, 0));
        while let Some(region_start) = to_visit_other.pop() {
            if self.grid.get(region_start).unwrap().visited {
                continue;
            }
            let mut to_visit: Vec<Vec2> = Vec::new();
            to_visit.push(region_start);
            let mut area = 0;
            let mut perimiter = 0;
            while let Some(v) = to_visit.pop() {
                let cell = self.grid.get(v).unwrap();
                if cell.visited {
                    continue;
                }
                perimiter += self.perimiter_for_cell(v) as usize;
                area += 1;
                for c in v.adjacents() {
                    let Some(adj) = self.grid.get(c) else {
                        continue;
                    };
                    if adj.visited {
                        continue;
                    }
                    if adj.plot == cell.plot {
                        to_visit.push(c);
                    } else {
                        to_visit_other.push(c);
                    }
                }
                self.grid.get_mut(v).unwrap().visited = true;
            }
            total += area * perimiter;
        }
        total
    }

    fn flood_fill2(&mut self) -> usize {
        let mut total = 0;
        let mut to_visit_other: Vec<Vec2> = Vec::new();
        to_visit_other.push(Vec2::new(0, 0));
        while let Some(region_start) = to_visit_other.pop() {
            if self.grid.get(region_start).unwrap().visited {
                continue;
            }
            let mut to_visit: Vec<Vec2> = Vec::new();
            to_visit.push(region_start);
            let mut area = 0;
            let mut sides = 0;
            while let Some(v) = to_visit.pop() {
                let cell = self.grid.get(v).unwrap();
                if cell.visited {
                    continue;
                }
                sides += self.count_new_sides_for_cell(v) as usize;
                area += 1;
                for c in v.adjacents() {
                    let Some(adj) = self.grid.get(c) else {
                        continue;
                    };
                    if adj.visited {
                        continue;
                    }
                    if adj.plot == cell.plot {
                        to_visit.push(c);
                    } else {
                        to_visit_other.push(c);
                    }
                }
                self.grid.get_mut(v).unwrap().visited = true;
            }
            total += area * sides;
        }
        total
    }

    fn perimiter_for_cell(&self, coord: Vec2) -> u8 {
        Direction::variants_as_array()
            .into_iter()
            .map(|d| if self.has_side(coord, d) { 1 } else { 0 })
            .sum()
    }

    fn count_new_sides_for_cell(&self, coord: Vec2) -> u8 {
        Direction::variants_as_array()
            .into_iter()
            .map(|d| {
                if !self.has_side(coord, d) {
                    return 0;
                }
                if self.has_side(coord, d.rotate_clockwise()) {
                    return 1;
                }
                if self.has_side(coord + d.rotate_clockwise().into(), d) {
                    return 0;
                }
                1
            })
            .sum()
    }

    fn has_side(&self, coord: Vec2, direction: Direction) -> bool {
        let Some(cell) = self.grid.get(coord) else {
            return false;
        };
        let adj = self.grid.get(coord + direction.into());
        Some(cell.plot) != adj.map(|c| c.plot)
    }
}

#[cfg(test)]
const INPUT1: &str = "AAAA
BBCD
BBCC
EEEC";

#[test]
fn practice_part1_input1() {
    assert_eq!(solve_part1(INPUT1), 140);
}

#[cfg(test)]
const INPUT2: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

#[test]
fn practice_part1_input2() {
    assert_eq!(solve_part1(INPUT2), 1930);
}

#[test]
fn practice_part2_input1() {
    assert_eq!(solve_part2(INPUT1), 80);
}

#[test]
fn practice_part2_input2() {
    assert_eq!(solve_part2(INPUT2), 1206);
}

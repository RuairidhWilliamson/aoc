use std::convert::Infallible;

use aoc_helper::grid::{Direction, Grid, Vec2};

pub fn solve_part1(input: &str) -> usize {
    let map = Map::parse(input);
    map.shortest_paths(2, 100)
}

pub fn solve_part2(input: &str) -> usize {
    let map = Map::parse(input);
    map.shortest_paths(20, 100)
}

struct Map {
    grid: Grid<Cell>,
    start: Vec2,
    end: Vec2,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut start = None;
        let mut end = None;
        let grid = Grid::parse_with(input, |coord, c| -> Result<Cell, Infallible> {
            match c {
                '.' => Ok(Cell::Empty),
                '#' => Ok(Cell::Wall),
                'S' => {
                    start = Some(coord);
                    Ok(Cell::Empty)
                }
                'E' => {
                    end = Some(coord);
                    Ok(Cell::Empty)
                }
                _ => panic!("unexpected character {c}"),
            }
        })
        .unwrap();
        let start = start.unwrap();
        let end = end.unwrap();
        Self { grid, start, end }
    }

    fn distance_map(&self, origin: Vec2) -> Grid<Option<u32>> {
        let mut grid = Grid::new_with_default(self.grid.width(), self.grid.height());
        *grid.get_mut(origin).unwrap() = Some(0);
        let mut to_visit = vec![origin];
        for i in 1.. {
            let grid = &mut grid;
            if to_visit.is_empty() {
                break;
            }
            to_visit = to_visit
                .iter()
                .flat_map(|pos| {
                    Direction::variants_as_array().map(|d| {
                        let new_pos = *pos + d.into();
                        if let Some(Cell::Empty) = self.grid.get(new_pos) {
                            let d = grid.get_mut(new_pos).unwrap();
                            if d.is_none() {
                                *d = Some(i);
                                Some(new_pos)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                })
                .flatten()
                .collect();
        }
        grid
    }

    fn shortest_paths(&self, cheat_time: isize, min_save: u32) -> usize {
        let distance_to_start = &self.distance_map(self.start);
        let distance_to_end = &self.distance_map(self.end);
        let no_cheat_time = distance_to_end.get(self.start).unwrap().unwrap();
        let max_time = no_cheat_time - min_save;
        distance_to_start
            .coords_iter()
            .filter_map(|pos| {
                let Some(t) = distance_to_start.get(pos).unwrap() else {
                    return None;
                };
                Some((-cheat_time..=cheat_time).flat_map(move |x| {
                    let limit = cheat_time - x.abs();
                    (-limit..=limit).map(move |y| {
                        let cheat_delta = Vec2::new(x, y);
                        let cheat_pos = pos + cheat_delta;
                        let Some(Some(d)) = distance_to_end.get(cheat_pos) else {
                            return false;
                        };
                        t + d + cheat_delta.l1_norm() as u32 <= max_time
                    })
                }))
            })
            .flatten()
            .filter(|b| *b)
            .count()
    }
}

enum Cell {
    Empty,
    Wall,
}

#[cfg(test)]
const INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

#[test]
fn practice_part1() {
    let map = Map::parse(INPUT);
    assert_eq!(map.shortest_paths(2, 64), 1);
    assert_eq!(map.shortest_paths(2, 2), 14 + 14 + 2 + 4 + 2 + 3 + 5);
}

#[test]
fn practice_part2() {
    let map = Map::parse(INPUT);
    assert_eq!(map.shortest_paths(20, 76), 3);
    assert_eq!(
        map.shortest_paths(20, 50),
        32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3
    );
}

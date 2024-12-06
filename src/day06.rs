use std::collections::HashSet;

use aoc_helper::grid::{Direction, Grid, Vec2};

pub fn solve_part1(input: &str) -> usize {
    let mut state = State::parse(input);
    state.guard_patrol();
    state.count_visited()
}

pub fn solve_part2(input: &str) -> usize {
    let original_state = State::parse(input);
    let mut first_pass_state = original_state.clone();
    first_pass_state.guard_patrol();
    let visited: HashSet<Vec2> = first_pass_state.iter_visited().collect();
    visited
        .into_iter()
        .filter(|&c| {
            let mut state = original_state.clone();
            *state.grid.get_mut(c).unwrap() = Cell::Obstacle;
            state.check_loop_guard_patrol()
        })
        .count()
}

#[derive(Debug, Clone)]
enum Cell {
    Empty(Visited),
    Obstacle,
}

#[derive(Clone)]
struct State {
    grid: Grid<Cell>,
    guard: Guard,
}

impl State {
    fn parse(input: &str) -> Self {
        let mut guard: Option<Vec2> = None;
        let width = input.lines().next().unwrap().len();
        let data: Vec<Cell> = input
            .lines()
            .enumerate()
            .flat_map(move |(y, l)| {
                l.chars().enumerate().map(move |(x, c)| {
                    (
                        Vec2 {
                            x: x as isize,
                            y: y as isize,
                        },
                        c,
                    )
                })
            })
            .map(|(coord, c)| match c {
                '.' => Cell::Empty(Visited::default()),
                '#' => Cell::Obstacle,
                '^' => {
                    debug_assert!(guard.is_none());
                    guard = Some(coord);
                    Cell::Empty(Visited([true, false, false, false]))
                }
                _ => panic!("unexpected character {c:?}"),
            })
            .collect();
        State {
            grid: Grid::new(data, width as isize),
            guard: Guard {
                position: guard.unwrap(),
                direction: Direction::North,
            },
        }
    }

    fn guard_patrol(&mut self) {
        loop {
            let new_position = self.guard.position + Vec2::from(self.guard.direction);
            let Some(cell) = self.grid.get_mut(new_position) else {
                break;
            };
            match cell {
                Cell::Empty(visited) => {
                    *visited.get_mut_direction(self.guard.direction) = true;
                    self.guard.position = new_position;
                }
                Cell::Obstacle => {
                    self.guard.direction = self.guard.direction.rotate_clockwise();
                }
            }
        }
    }

    fn check_loop_guard_patrol(&mut self) -> bool {
        loop {
            let new_position = self.guard.position + Vec2::from(self.guard.direction);
            let Some(cell) = self.grid.get_mut(new_position) else {
                return false;
            };
            match cell {
                Cell::Empty(visited) => {
                    let v = visited.get_mut_direction(self.guard.direction);
                    if *v {
                        return true;
                    }
                    *v = true;
                    self.guard.position = new_position;
                }
                Cell::Obstacle => {
                    self.guard.direction = self.guard.direction.rotate_clockwise();
                }
            }
        }
    }

    fn count_visited(&self) -> usize {
        self.grid
            .flat_iter()
            .filter(|c| matches!(c, Cell::Empty(v) if v.any()))
            .count()
    }

    fn iter_visited(&self) -> impl Iterator<Item = Vec2> + use<'_> {
        self.grid
            .coords_iter()
            .filter(|c| matches!(self.grid.get(*c).unwrap(), Cell::Empty(v) if v.any()))
    }
}

#[derive(Debug, Default, Clone)]
struct Visited([bool; 4]);

impl Visited {
    fn any(&self) -> bool {
        self.0.iter().any(|x| *x)
    }

    fn get_mut_direction(&mut self, d: Direction) -> &mut bool {
        &mut self.0[u8::from(d) as usize]
    }
}

#[derive(Clone)]
struct Guard {
    position: Vec2,
    direction: Direction,
}

#[cfg(test)]
const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

#[test]
fn practice_part1() {
    assert_eq!(solve_part1(INPUT), 41);
}

#[test]
fn practice_part2() {
    assert_eq!(solve_part2(INPUT), 6);
}

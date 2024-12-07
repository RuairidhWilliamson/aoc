use aoc_helper::grid::{Direction, Grid, Vec2};

pub fn solve_part1(input: &str) -> usize {
    let mut state = State::parse(input);
    while !state.guard_patrol_step().is_done() {}
    state.count_visited()
}

pub fn solve_part2(input: &str) -> usize {
    let mut first_pass_state = State::parse(input);
    let mut scratch = first_pass_state.clone();
    let mut count = 0;
    loop {
        let step_result = first_pass_state.guard_patrol_step();
        match step_result {
            StepResult::Loop | StepResult::OffGrid => {
                break;
            }
            StepResult::New => {
                first_pass_state.clone_into(&mut scratch);
                *scratch
                    .grid
                    .get_mut(first_pass_state.guard.position)
                    .unwrap() = Cell::Obstacle;
                scratch.guard.position -= Vec2::from(scratch.guard.direction);
                if scratch.check_loop_guard_patrol() {
                    count += 1;
                }
            }
            _ => (),
        }
    }
    count
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
        let data: Box<[Cell]> = input
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
                    let mut v = Visited::default();
                    v.set_visited_direction(Direction::North);
                    Cell::Empty(v)
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

    fn check_loop_guard_patrol(&mut self) -> bool {
        loop {
            let step_result = self.guard_patrol_step();
            match step_result {
                StepResult::Loop => return true,
                StepResult::OffGrid => return false,
                _ => (),
            }
        }
    }

    fn guard_patrol_step(&mut self) -> StepResult {
        let new_position = self.guard.position + Vec2::from(self.guard.direction);
        let Some(cell) = self.grid.get_mut(new_position) else {
            return StepResult::OffGrid;
        };
        match cell {
            Cell::Empty(visited) => {
                let has_visited_at_all = visited.any();
                let v = visited.get_direction(self.guard.direction);
                if v {
                    return StepResult::Loop;
                }
                visited.set_visited_direction(self.guard.direction);
                self.guard.position = new_position;
                if has_visited_at_all {
                    StepResult::Visited
                } else {
                    StepResult::New
                }
            }
            Cell::Obstacle => {
                self.guard.direction = self.guard.direction.rotate_clockwise();
                StepResult::Rotated
            }
        }
    }

    fn count_visited(&self) -> usize {
        self.grid
            .flat_iter()
            .filter(|c| matches!(c, Cell::Empty(v) if v.any()))
            .count()
    }
}

enum StepResult {
    New,
    Rotated,
    Visited,
    Loop,
    OffGrid,
}

impl StepResult {
    fn is_done(&self) -> bool {
        matches!(self, Self::Loop | Self::OffGrid)
    }
}

#[derive(Debug, Default, Clone)]
struct Visited(u8);

impl Visited {
    fn any(&self) -> bool {
        self.0 != 0
    }

    fn get_direction(&self, d: Direction) -> bool {
        (self.0 >> u8::from(d)) & 1 == 1
    }

    fn set_visited_direction(&mut self, d: Direction) {
        self.0 |= 1 << u8::from(d)
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

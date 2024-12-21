use std::{collections::HashMap, convert::Infallible};

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

    fn shortest_paths(&self, cheat_time: usize, min_save: usize) -> usize {
        let fair_map = FairMap::new(self);
        let no_cheat_time = fair_map.grid.get(self.start).unwrap().unwrap();
        let max_time = no_cheat_time - min_save;

        let init = State {
            position: self.start,
            cheat: CheatState::Unused,
        };
        let mut open = Vec::new();
        let mut time_map = HashMap::new();
        time_map.insert(init, 0);
        open.push(init);
        let cheats = HashMap::new();
        let mut searcher = Searcher {
            map: self,
            fair_map: &fair_map,
            open,
            time_map,
            cheats,
            max_time,
            cheat_time,
        };
        searcher.shortest_paths();
        (searcher.cheats).len()
    }
}

enum Cell {
    Empty,
    Wall,
}

struct FairMap {
    grid: Grid<Option<usize>>,
}

impl FairMap {
    fn new(map: &Map) -> Self {
        let mut grid = Grid::<Option<usize>>::new_with_default(map.grid.width(), map.grid.height());
        *grid.get_mut(map.end).unwrap() = Some(0);
        let mut to_visit = vec![map.end];
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
                        if let Some(Cell::Empty) = map.grid.get(new_pos) {
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
        Self { grid }
    }
}

struct Searcher<'a> {
    map: &'a Map,
    fair_map: &'a FairMap,
    open: Vec<State>,
    time_map: HashMap<State, usize>,
    cheats: HashMap<(Vec2, Vec2), usize>,
    max_time: usize,
    cheat_time: usize,
}

impl Searcher<'_> {
    fn shortest_paths(&mut self) {
        while let Some(q) = self.open.pop() {
            let time = *self.time_map.get(&q).unwrap();
            let new_time = time + 1;
            for d in Direction::variants_as_array() {
                let position = q.position + d.into();
                match self.map.grid.get(position) {
                    None => continue,
                    Some(Cell::Empty) => {
                        match q.cheat {
                            CheatState::Unused => {
                                self.visit_state(
                                    State {
                                        position,
                                        cheat: CheatState::Unused,
                                    },
                                    new_time,
                                );
                                self.visit_state(
                                    State {
                                        position,
                                        cheat: CheatState::Active {
                                            start: q.position,
                                            length: 1,
                                        },
                                    },
                                    new_time,
                                );
                            }
                            CheatState::Active { start, length }
                                if length + 1 >= self.cheat_time =>
                            {
                                self.visit_finish_cheat(start, position, new_time);
                            }
                            CheatState::Active { start, length } => {
                                self.visit_finish_cheat(start, position, new_time);
                                self.visit_state(
                                    State {
                                        position,
                                        cheat: CheatState::Active {
                                            start,
                                            length: length + 1,
                                        },
                                    },
                                    new_time,
                                )
                            }
                        };
                    }
                    Some(Cell::Wall) => {
                        match q.cheat {
                            CheatState::Unused => self.visit_state(
                                State {
                                    position,
                                    cheat: CheatState::Active {
                                        start: q.position,
                                        length: 1,
                                    },
                                },
                                new_time,
                            ),
                            CheatState::Active { start: _, length }
                                if length + 1 >= self.cheat_time => {}
                            CheatState::Active { start, length } => self.visit_state(
                                State {
                                    position,
                                    cheat: CheatState::Active {
                                        start,
                                        length: length + 1,
                                    },
                                },
                                new_time,
                            ),
                        };
                    }
                };
            }
        }
    }

    fn visit_state(&mut self, new_q: State, new_time: usize) {
        let old_time = self.time_map.entry(new_q).or_insert(usize::MAX);
        if new_time >= *old_time {
            return;
        }
        *old_time = new_time;
        if self.max_time > new_time
            && (new_q.position - self.map.end).l1_norm() <= self.max_time - new_time
        {
            self.open.push(new_q);
        }
    }

    fn visit_finish_cheat(&mut self, start: Vec2, position: Vec2, new_time: usize) {
        let distance = self.fair_map.grid.get(position).unwrap().unwrap();
        if new_time + distance <= self.max_time {
            let best = self.cheats.entry((start, position)).or_insert(usize::MAX);
            *best = (*best).min(new_time + distance);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    position: Vec2,
    cheat: CheatState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CheatState {
    Unused,
    Active { start: Vec2, length: usize },
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
    assert_eq!(map.shortest_paths(2, 6), 16);
    assert_eq!(map.shortest_paths(2, 8), 14);
    assert_eq!(map.shortest_paths(2, 10), 10);
    assert_eq!(map.shortest_paths(2, 12), 8);
    assert_eq!(map.shortest_paths(2, 20), 5);
    assert_eq!(map.shortest_paths(2, 36), 4);
    assert_eq!(map.shortest_paths(2, 38), 3);
    assert_eq!(map.shortest_paths(2, 40), 2);
    assert_eq!(map.shortest_paths(2, 64), 1);
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

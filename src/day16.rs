use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    convert::Infallible,
};

use aoc_helper::grid::{Direction, Grid, Vec2};

pub fn solve_part1(input: &str) -> usize {
    let map = Map::parse(input);
    map.search()
}

pub fn solve_part2(input: &str) -> usize {
    let map = Map::parse(input);
    map.search_all()
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
        let grid = Grid::parse_with::<Infallible>(input, |p, c| match c {
            '.' => Ok(Cell::Empty),
            '#' => Ok(Cell::Wall),
            'E' => {
                end = Some(p);
                Ok(Cell::Empty)
            }
            'S' => {
                start = Some(p);
                Ok(Cell::Empty)
            }
            _ => panic!("unexpected character {c:?}"),
        })
        .unwrap();
        let start = start.unwrap();
        let end = end.unwrap();
        Self { grid, start, end }
    }

    fn search(&self) -> usize {
        let init_state = State(self.start, Direction::East);
        let mut g_map = HashMap::new();
        g_map.insert(init_state.clone(), 0);
        let mut open = BinaryHeap::new();
        open.push(Reverse(StateWithCost(0, init_state)));
        while let Some(Reverse(StateWithCost(_, q))) = open.pop() {
            if q.0 == self.end {
                return *g_map.get(&q).unwrap();
            }
            let g = g_map.get(&q).unwrap();
            for (new_g, s) in [
                (g + 1, State(q.0 + q.1.into(), q.1)),
                (g + 1000, State(q.0, q.1.rotate_clockwise())),
                (g + 1000, State(q.0, q.1.rotate_anticlockwise())),
            ] {
                if let Cell::Wall = self.grid.get(s.0).unwrap() {
                    continue;
                }
                let old_g = g_map.entry(s.clone()).or_insert(usize::MAX);
                if new_g < *old_g {
                    *old_g = new_g;
                    let f = new_g + (s.0 - self.end).l1_norm();
                    open.push(Reverse(StateWithCost(f, s)));
                }
            }
        }
        todo!()
    }

    fn search_all(&self) -> usize {
        let init_state = State(self.start, Direction::East);
        let mut g_map = HashMap::new();
        g_map.insert(init_state.clone(), 0);
        let mut open = BinaryHeap::new();
        open.push(Reverse(StateWithCost(0, init_state)));
        let mut lowest_cost: Option<usize> = None;
        while let Some(Reverse(StateWithCost(_, q))) = open.pop() {
            if q.0 == self.end {
                lowest_cost = Some(*g_map.get(&q).unwrap());
                continue;
            }
            if let Some(lowest_cost) = lowest_cost {
                if *g_map.get(&q).unwrap() > lowest_cost {
                    return self.reconstruct(lowest_cost, &g_map);
                }
            }
            let g = g_map.get(&q).unwrap();
            for (new_g, s) in [
                (g + 1, State(q.0 + q.1.into(), q.1)),
                (g + 1000, State(q.0, q.1.rotate_clockwise())),
                (g + 1000, State(q.0, q.1.rotate_anticlockwise())),
            ] {
                if let Cell::Wall = self.grid.get(s.0).unwrap() {
                    continue;
                }
                let old_g = g_map.entry(s.clone()).or_insert(usize::MAX);
                if new_g < *old_g {
                    *old_g = new_g;
                    let f = new_g + (s.0 - self.end).l1_norm();
                    open.push(Reverse(StateWithCost(f, s)));
                }
            }
        }
        unreachable!()
    }

    fn reconstruct(&self, final_cost: usize, g_map: &HashMap<State, usize>) -> usize {
        let mut covered = HashSet::<Vec2>::new();
        covered.insert(self.end);
        let mut states: Vec<StateWithCost> = Direction::variants_as_array()
            .into_iter()
            .map(|d| State(self.end, d))
            .filter(|q| g_map.get(q).is_some_and(|c| final_cost == *c))
            .map(|q| StateWithCost(final_cost, q))
            .collect();
        while !states.is_empty() {
            states = states
                .into_iter()
                .flat_map(|StateWithCost(g, q)| {
                    [
                        StateWithCost(g.saturating_sub(1), State(q.0 - q.1.into(), q.1)),
                        StateWithCost(g.saturating_sub(1000), State(q.0, q.1.rotate_clockwise())),
                        StateWithCost(
                            g.saturating_sub(1000),
                            State(q.0, q.1.rotate_anticlockwise()),
                        ),
                    ]
                })
                .filter(|StateWithCost(g, q)| g_map.get(q).is_some_and(|c| c == g))
                .inspect(|StateWithCost(_, q)| {
                    covered.insert(q.0);
                })
                .collect();
        }
        covered.len()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct StateWithCost(usize, State);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State(Vec2, Direction);

enum Cell {
    Empty,
    Wall,
}

#[cfg(test)]
const INPUT1: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

#[cfg(test)]
const INPUT2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

#[test]
fn practice_part1_example1() {
    assert_eq!(solve_part1(INPUT1), 7036);
}

#[test]
fn practice_part1_example2() {
    assert_eq!(solve_part1(INPUT2), 11048);
}

#[test]
fn practice_part2_example1() {
    assert_eq!(solve_part2(INPUT1), 45);
}

#[test]
fn practice_part2_example2() {
    assert_eq!(solve_part2(INPUT2), 64);
}

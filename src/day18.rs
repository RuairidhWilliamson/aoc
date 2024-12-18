use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use aoc_helper::grid::{Direction, Grid, Vec2};

pub fn solve_part1(input: &str) -> usize {
    Map::new(71, parse(input)).shortest_path(1024).unwrap()
}

pub fn solve_part2(input: &str) -> Vec2 {
    let i = Map::new(71, parse(input)).first_byte_no_route();
    parse(input).nth(i).unwrap()
}

fn parse(input: &str) -> impl Iterator<Item = Vec2> + use<'_> {
    input.lines().map(|l| {
        let (x, y) = l.split_once(',').unwrap();
        Vec2::new(x.parse().unwrap(), y.parse().unwrap())
    })
}

enum Cell {
    Empty,
    Corrupted(usize),
}

struct Map(Grid<Cell>);

impl Map {
    fn new(size: usize, mem: impl Iterator<Item = Vec2>) -> Self {
        let mut grid = Grid::new(
            (0..size * size).map(|_| Cell::Empty).collect(),
            size as isize,
        );
        for (i, m) in mem.enumerate() {
            *grid.get_mut(m).unwrap() = Cell::Corrupted(i);
        }
        Self(grid)
    }

    fn shortest_path(&self, time: usize) -> Option<usize> {
        let init_state = Vec2::new(0, 0);
        let mut open = BinaryHeap::new();
        let mut g_map = HashMap::new();
        g_map.insert(init_state, 0);
        open.push(Reverse(PosWithF(0, init_state)));
        while let Some(Reverse(PosWithF(_, q))) = open.pop() {
            let g = *g_map.get(&q).unwrap();
            if q == self.end() {
                return Some(g);
            }
            for d in Direction::variants_as_array() {
                let pos = q + d.into();
                match self.0.get(pos) {
                    Some(Cell::Empty) => {}
                    Some(Cell::Corrupted(t)) if *t >= time => {}
                    _ => continue,
                }
                let new_g = g + 1;
                let old_g = g_map.entry(pos).or_insert(usize::MAX);
                if new_g < *old_g {
                    *old_g = new_g;
                    let f = new_g + (pos - self.end()).l1_norm();
                    open.push(Reverse(PosWithF(f, pos)));
                }
            }
        }
        None
    }

    fn end(&self) -> Vec2 {
        Vec2::new(self.0.width() - 1, self.0.height() - 1)
    }

    fn first_byte_no_route(&self) -> usize {
        let mut rng = 0..(self.0.width() * self.0.height()) as usize;
        debug_assert!(self.shortest_path(rng.start).is_some());
        debug_assert!(self.shortest_path(rng.end).is_none());
        loop {
            let time = (rng.end + rng.start) / 2;
            if time == rng.start || time == rng.end {
                debug_assert!(self.shortest_path(rng.start).is_some());
                debug_assert!(self.shortest_path(rng.end).is_none());
                return rng.start;
            }
            if self.shortest_path(time).is_some() {
                rng.start = time;
            } else {
                rng.end = time;
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct PosWithF(usize, Vec2);

#[cfg(test)]
const INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

#[test]
fn practice_part1() {
    assert_eq!(Map::new(7, parse(INPUT)).shortest_path(12).unwrap(), 22);
}

#[test]
fn practice_part2() {
    let i = Map::new(7, parse(INPUT)).first_byte_no_route();

    assert_eq!(parse(INPUT).nth(i).unwrap(), Vec2::new(6, 1));
}

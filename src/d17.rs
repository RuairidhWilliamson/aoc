use std::collections::{BinaryHeap, HashMap};

use crate::{
    common::grid::{add_coords, Coord, Grid},
    PartFn,
};

pub const PARTS: (PartFn, PartFn) = (part1, part2);

fn part1(input: &str) -> isize {
    let city: Grid<CellHeatLoss> = input.trim().trim_matches('\n').parse().unwrap();
    find_route::<NormalCrucibleState>(&city) as isize
}

fn part2(input: &str) -> isize {
    let city: Grid<CellHeatLoss> = input.trim().trim_matches('\n').parse().unwrap();
    find_route::<UltraCrucibleState>(&city) as isize
}

fn find_route<T>(city: &Grid<CellHeatLoss>) -> usize
where
    T: CrucibleState,
{
    let mut costs = HashMap::<T, usize>::new();
    let mut prev = HashMap::<T, T>::new();
    let mut heap = BinaryHeap::<StateWithCost<T>>::new();
    let start_states = T::start_states();
    for start_state in start_states.clone() {
        heap.push(StateWithCost {
            cost: 0,
            state: start_state,
        });
    }
    while let Some(StateWithCost { state, cost }) = heap.pop() {
        if state.is_end(city) {
            // Backtracking to print path

            #[derive(Clone, Copy)]
            enum Path {
                Path,
                NoPath,
            }

            impl From<&Path> for char {
                fn from(value: &Path) -> Self {
                    match value {
                        Path::Path => '#',
                        Path::NoPath => '.',
                    }
                }
            }
            let mut grid = Grid::new(Path::NoPath, city.width() as usize, city.height() as usize);
            let mut s = state;
            while let Some(next_s) = prev.remove(&s) {
                let c = grid.get_mut(s.position()).unwrap();
                *c = Path::Path;
                if start_states.contains(&next_s) {
                    break;
                }
                s = next_s;
            }
            println!("{grid}");
            return cost;
        }

        if &cost > costs.get(&state).unwrap_or(&usize::MAX) {
            continue;
        }

        state
            .next_states()
            .into_iter()
            .filter_map(|state| {
                let c = city.get(state.position())?;
                Some(StateWithCost {
                    state,
                    cost: cost + c.0,
                })
            })
            .for_each(|sc| {
                if &sc.cost < costs.get(&sc.state).unwrap_or(&usize::MAX) {
                    costs.insert(sc.state.clone(), sc.cost);
                    prev.insert(sc.state.clone(), state.clone());
                    heap.push(sc);
                }
            });
    }
    panic!("did not reach end")
}

trait CrucibleState: std::fmt::Debug + Clone + core::hash::Hash + Ord + Sized + 'static {
    fn start_states() -> Vec<Self>;
    fn is_end(&self, city: &Grid<CellHeatLoss>) -> bool;
    fn position(&self) -> Coord;
    fn next_states(&self) -> Vec<Self>;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct NormalCrucibleState {
    position: Coord,
    direction: Direction,
    straight_line: usize,
}

impl CrucibleState for NormalCrucibleState {
    fn start_states() -> Vec<Self> {
        vec![
            Self {
                position: (0, 0),
                direction: Direction::East,
                straight_line: 0,
            },
            Self {
                position: (0, 0),
                direction: Direction::South,
                straight_line: 0,
            },
        ]
    }

    fn is_end(&self, city: &Grid<CellHeatLoss>) -> bool {
        self.position == (city.width() - 1, city.height() - 1)
    }

    fn position(&self) -> Coord {
        self.position
    }

    fn next_states(&self) -> Vec<Self> {
        [
            self.direction,
            self.direction.left(),
            self.direction.right(),
        ]
        .into_iter()
        .map(|direction| Self {
            position: add_coords(self.position, direction.rel_coord()),
            direction,
            straight_line: if direction == self.direction {
                self.straight_line + 1
            } else {
                0
            },
        })
        .filter(|state| state.straight_line < 3)
        .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct UltraCrucibleState {
    position: Coord,
    direction: Direction,
    straight_line: usize,
}

impl CrucibleState for UltraCrucibleState {
    fn start_states() -> Vec<Self> {
        vec![
            Self {
                position: (0, 0),
                direction: Direction::East,
                straight_line: 0,
            },
            Self {
                position: (0, 0),
                direction: Direction::South,
                straight_line: 0,
            },
        ]
    }

    fn is_end(&self, city: &Grid<CellHeatLoss>) -> bool {
        self.straight_line >= 3 && self.position == (city.width() - 1, city.height() - 1)
    }

    fn position(&self) -> Coord {
        self.position
    }

    fn next_states(&self) -> Vec<Self> {
        [
            self.direction,
            self.direction.left(),
            self.direction.right(),
        ]
        .into_iter()
        .map(|direction| Self {
            position: add_coords(self.position, direction.rel_coord()),
            direction,
            straight_line: if direction == self.direction {
                self.straight_line + 1
            } else {
                0
            },
        })
        .filter(|state| {
            (state.straight_line == 0 && self.straight_line >= 3)
                || (state.straight_line != 0 && state.straight_line < 10)
        })
        .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct StateWithCost<T> {
    state: T,
    cost: usize,
}

impl<T> Ord for StateWithCost<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.state.cmp(&other.state))
    }
}

impl<T> PartialOrd for StateWithCost<T>
where
    T: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rel_coord(&self) -> Coord {
        match self {
            Self::North => (0, -1),
            Self::East => (1, 0),
            Self::South => (0, 1),
            Self::West => (-1, 0),
        }
    }

    fn left(&self) -> Direction {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }

    fn right(&self) -> Direction {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

struct CellHeatLoss(usize);

impl TryFrom<char> for CellHeatLoss {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value.is_ascii_digit() {
            return Ok(CellHeatLoss((value as u8 - b'0') as usize));
        }
        Err(())
    }
}

#[test]
fn example1() {
    let input = "
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
    ";
    assert_eq!(part1(input), 102);
    assert_eq!(part2(input), 94);
}

#[test]
fn example2() {
    let input = "
111111111111
999999999991
999999999991
999999999991
999999999991
    ";
    assert_eq!(part2(input), 71);
}

#[test]
fn example4() {
    let input = "
91111111
11111111
11111111
11111111
11111111
    ";
    assert_eq!(part2(input), 11);
}

#[test]
fn example5() {
    let input = "
9999999999919999999
9999999999919999999
9999999999919999999
9999999999919999999
9999999999919999999
9999999999919999999
9999999999919999999
    ";
    assert_eq!(part2(input), 208);
}

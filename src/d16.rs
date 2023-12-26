use crate::{
    common::grid::{add_coords, Coord, Grid},
    PartFn,
};

pub const PARTS: (PartFn, PartFn) = (part1, part2);

fn part1(input: &str) -> usize {
    let grid = parse_grid(input);
    let energized = trace_laser(
        &grid,
        LaserState {
            pos: (0, 0),
            direction: Direction::East,
        },
    );
    count_energized(&energized)
}

fn part2(input: &str) -> usize {
    let grid = parse_grid(input);

    (0..grid.width())
        .map(|x| {
            [
                LaserState {
                    pos: (x, 0),
                    direction: Direction::South,
                },
                LaserState {
                    pos: (x, grid.height() - 1),
                    direction: Direction::North,
                },
            ]
        })
        .chain((0..grid.height()).map(|y| {
            [
                LaserState {
                    pos: (0, y),
                    direction: Direction::East,
                },
                LaserState {
                    pos: (grid.width() - 1, y),
                    direction: Direction::West,
                },
            ]
        }))
        .flatten()
        .map(|start| {
            let energized = trace_laser(&grid, start);
            count_energized(&energized)
        })
        .max()
        .unwrap()
}

fn parse_grid(input: &str) -> Grid<Cell> {
    input.trim().trim_matches('\n').parse().unwrap()
}

fn trace_laser(grid: &Grid<Cell>, start: LaserState) -> Grid<Energized> {
    let mut energized = Grid::new(
        Energized::NotEnergized,
        grid.width() as usize,
        grid.height() as usize,
    );
    let mut states = vec![start];
    // Only counts splitters we have split on
    let mut visited_splitters = Grid::new(false, grid.width() as usize, grid.height() as usize);
    for _ in 0..10000 {
        if states.is_empty() {
            break;
        }
        states.iter().for_each(|s| {
            if let Some(e) = energized.get_mut(s.pos) {
                *e = Energized::Energized;
            }
        });

        states = states
            .into_iter()
            .flat_map(|s| s.update_direction(grid, &mut visited_splitters))
            .map(|mut s| {
                s.update_position();
                s
            })
            .collect();
    }
    energized
}

fn count_energized(energized: &Grid<Energized>) -> usize {
    energized
        .enumerate_coords()
        .filter(|c| energized.get(*c).unwrap() == &Energized::Energized)
        .count()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LaserState {
    pos: Coord,
    direction: Direction,
}

impl LaserState {
    fn update_direction(
        self,
        grid: &Grid<Cell>,
        visited_splitters: &mut Grid<bool>,
    ) -> Vec<LaserState> {
        match grid.get(self.pos) {
            None => vec![],
            Some(Cell::Empty) => vec![self],
            Some(Cell::Splitter(dir)) => {
                if self.direction.splitter_direction() == *dir {
                    vec![self]
                } else {
                    let visited = visited_splitters.get_mut(self.pos).unwrap();
                    if *visited {
                        vec![]
                    } else {
                        *visited = true;
                        dir.directions()
                            .into_iter()
                            .map(|d| Self {
                                pos: self.pos,
                                direction: d,
                            })
                            .collect()
                    }
                }
            }
            Some(Cell::Mirror(dir)) => {
                vec![Self {
                    pos: self.pos,
                    direction: dir.transform(&self.direction),
                }]
            }
        }
    }

    fn update_position(&mut self) {
        self.pos = add_coords(self.pos, self.direction.rel_coord());
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Energized {
    #[default]
    NotEnergized,
    Energized,
}

impl From<&Energized> for char {
    fn from(value: &Energized) -> Self {
        match value {
            Energized::NotEnergized => '.',
            Energized::Energized => '#',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    fn splitter_direction(&self) -> SplitterDirection {
        match self {
            Self::North => SplitterDirection::Vertical,
            Self::East => SplitterDirection::Horizontal,
            Self::South => SplitterDirection::Vertical,
            Self::West => SplitterDirection::Horizontal,
        }
    }
}

enum Cell {
    Splitter(SplitterDirection),
    Mirror(MirrorDirection),
    Empty,
}

impl TryFrom<char> for Cell {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '\\' => Ok(Cell::Mirror(MirrorDirection::Positive)),
            '/' => Ok(Cell::Mirror(MirrorDirection::Negative)),
            '-' => Ok(Cell::Splitter(SplitterDirection::Horizontal)),
            '|' => Ok(Cell::Splitter(SplitterDirection::Vertical)),
            '.' => Ok(Cell::Empty),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Eq)]
enum SplitterDirection {
    Horizontal,
    Vertical,
}

impl SplitterDirection {
    fn directions(&self) -> [Direction; 2] {
        match self {
            SplitterDirection::Horizontal => [Direction::East, Direction::West],
            SplitterDirection::Vertical => [Direction::North, Direction::South],
        }
    }
}

enum MirrorDirection {
    Positive,
    Negative,
}

impl MirrorDirection {
    fn transform(&self, d: &Direction) -> Direction {
        match (d, self) {
            (Direction::North, MirrorDirection::Positive) => Direction::West,
            (Direction::North, MirrorDirection::Negative) => Direction::East,
            (Direction::East, MirrorDirection::Positive) => Direction::South,
            (Direction::East, MirrorDirection::Negative) => Direction::North,
            (Direction::South, MirrorDirection::Positive) => Direction::East,
            (Direction::South, MirrorDirection::Negative) => Direction::West,
            (Direction::West, MirrorDirection::Positive) => Direction::North,
            (Direction::West, MirrorDirection::Negative) => Direction::South,
        }
    }
}

#[test]
fn example1() {
    let input = r"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
    ";
    assert_eq!(part1(input), 46);
}

#[test]
fn example2() {
    let input = r"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
    ";
    assert_eq!(part2(input), 51);
}

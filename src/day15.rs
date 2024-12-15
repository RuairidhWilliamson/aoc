use aoc_helper::grid::{Direction, DisplayableChar, Grid, Vec2};

pub fn solve_part1(input: &str) -> usize {
    let (mut map, moves) = parse_puzzle(input);
    map.perform_moves(moves);
    map.calc_gps()
}

pub fn solve_part2(input: &str) -> usize {
    let (map, moves) = parse_puzzle(input);
    let mut map = Map2::from_map1(map);
    map.perform_moves(moves);
    map.calc_gps()
}

fn parse_puzzle(input: &str) -> (Map, impl Iterator<Item = Direction> + use<'_>) {
    let (map, moves) = input.split_once("\n\n").unwrap();
    let map = Map::parse(map);
    let moves = parse_moves(moves);
    (map, moves)
}

struct Map {
    grid: Grid<Cell>,
    robot: Vec2,
}

impl Map {
    fn parse(map_input: &str) -> Self {
        let mut width = None;
        if !map_input.lines().all(|l| {
            let w = l.chars().count() as isize;
            if let Some(expected_width) = width {
                expected_width == w
            } else {
                width = Some(w);
                true
            }
        }) {
            panic!("rows different widths");
        }
        let width = width.unwrap();
        let mut robot = None;
        let data: Box<[Cell]> = map_input
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(move |(x, c)| (Vec2::new(x as isize, y as isize), c))
            })
            .map(|(coord, c)| match c {
                '.' => Cell::Empty,
                'O' => Cell::Box,
                '#' => Cell::Wall,
                '@' => {
                    robot = Some(coord);
                    Cell::Empty
                }
                _ => panic!("unexpected character {c:?}"),
            })
            .collect();
        let robot = robot.unwrap();
        Self {
            grid: Grid::new(data, width),
            robot,
        }
    }

    fn perform_move(&mut self, m: Direction) {
        let mut c = self.robot;
        loop {
            c += m.into();
            match self.grid.get(c).unwrap() {
                Cell::Empty => break,
                Cell::Box => (),
                Cell::Wall => return,
            }
        }
        loop {
            if c - m.into() == self.robot {
                self.robot = c;
                break;
            }
            self.grid.swap(c, c - m.into());
            c -= m.into();
        }
    }

    fn perform_moves(&mut self, iter: impl Iterator<Item = Direction>) {
        for m in iter {
            self.perform_move(m);
        }
    }

    fn calc_gps(&self) -> usize {
        self.grid
            .coords_iter()
            .map(|c| {
                let cell = self.grid.get(c).unwrap();
                match cell {
                    Cell::Box => (c.x + c.y * 100) as usize,
                    _ => 0,
                }
            })
            .sum()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Cell {
    Empty,
    Box,
    Wall,
}

fn parse_moves(s: &str) -> impl Iterator<Item = Direction> + use<'_> {
    s.chars().filter_map(|c| match c {
        '<' => Some(Direction::West),
        '^' => Some(Direction::North),
        '>' => Some(Direction::East),
        'v' => Some(Direction::South),
        '\n' => None,
        _ => panic!("unexpected character {c:?}"),
    })
}

struct Map2 {
    grid: Grid<Cell2>,
    robot: Vec2,
}

impl Map2 {
    fn from_map1(map: Map) -> Self {
        let grid = Grid::new(
            map.grid
                .flat_iter()
                .flat_map(|c| match c {
                    Cell::Empty => [Cell2::Empty, Cell2::Empty],
                    Cell::Wall => [Cell2::Wall, Cell2::Wall],
                    Cell::Box => [Cell2::BoxL, Cell2::BoxR],
                })
                .collect(),
            map.grid.width() * 2,
        );
        Self {
            grid,
            robot: map.robot * Vec2::new(2, 1),
        }
    }

    fn perform_move(&mut self, m: Direction) {
        debug_assert_eq!(self.grid.get(self.robot).unwrap(), &Cell2::Empty);
        // println!("perform move {:?} {:?}", self.robot, m);
        if !self.can_move(self.robot, m) {
            return;
        }
        self.do_move(self.robot, m);
        self.robot += m.into();
    }

    fn can_move(&self, mut c: Vec2, m: Direction) -> bool {
        c += m.into();
        match (self.grid.get(c).unwrap(), m) {
            (Cell2::Empty, _) => true,
            (Cell2::Wall, _) => false,
            (Cell2::BoxL, Direction::North | Direction::South) => {
                self.can_move(c, m) && self.can_move(c + Vec2::new(1, 0), m)
            }
            (Cell2::BoxR, Direction::North | Direction::South) => {
                self.can_move(c, m) && self.can_move(c - Vec2::new(1, 0), m)
            }
            (Cell2::BoxL | Cell2::BoxR, Direction::East | Direction::West) => self.can_move(c, m),
        }
    }

    fn do_move(&mut self, mut c: Vec2, m: Direction) {
        c += m.into();
        match (self.grid.get(c).unwrap(), m) {
            (Cell2::Empty, _) => {
                self.grid.swap(c, c - m.into());
            }
            (Cell2::Wall, _) => unreachable!(),
            (Cell2::BoxL, Direction::North | Direction::South) => {
                self.do_move(c, m);
                self.do_move(c + Vec2::new(1, 0), m);
                self.grid.swap(c, c - m.into());
            }
            (Cell2::BoxR, Direction::North | Direction::South) => {
                self.do_move(c, m);
                self.do_move(c - Vec2::new(1, 0), m);
                self.grid.swap(c, c - m.into());
            }
            (Cell2::BoxL | Cell2::BoxR, Direction::East | Direction::West) => {
                self.do_move(c, m);
                self.grid.swap(c, c - m.into());
            }
        }
    }

    fn perform_moves(&mut self, iter: impl Iterator<Item = Direction>) {
        for m in iter {
            self.perform_move(m);
            // println!("{}", &self.grid);
            // let mut s = String::new();
            // std::io::stdin().read_line(&mut s).unwrap();
        }
    }

    fn calc_gps(&self) -> usize {
        self.grid
            .coords_iter()
            .map(|c| {
                let cell = self.grid.get(c).unwrap();
                match cell {
                    Cell2::BoxL => (c.x + c.y * 100) as usize,
                    _ => 0,
                }
            })
            .sum()
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Cell2 {
    Empty,
    BoxL,
    BoxR,
    Wall,
}

impl DisplayableChar for Cell2 {
    fn display_as_char(&self) -> char {
        match self {
            Cell2::Empty => '.',
            Cell2::BoxL => '[',
            Cell2::BoxR => ']',
            Cell2::Wall => '#',
        }
    }
}

#[cfg(test)]
const INPUT_SMALL: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

#[cfg(test)]
const INPUT_LARGE: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

#[test]
fn practice_part1_small() {
    assert_eq!(solve_part1(INPUT_SMALL), 2028);
}

#[test]
fn practice_part1_large() {
    assert_eq!(solve_part1(INPUT_LARGE), 10092);
}

// #[test]
// fn practice_part2_small() {
//     assert_eq!(
//         solve_part2(
//             "#######
// #...#.#
// #.....#
// #..OO@#
// #..O..#
// #.....#
// #######

// <vv<<^^<<^^"
//         ),
//         0
//     );
// }

#[test]
fn practice_part2_large() {
    assert_eq!(solve_part2(INPUT_LARGE), 9021);
}

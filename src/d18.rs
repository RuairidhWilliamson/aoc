use std::str::FromStr;

use crate::PartFn;

pub const PARTS: (PartFn, PartFn) = (part1, part2);

fn part1(input: &str) -> usize {
    let instructions = get_instructions(input, false, false);
    area(instructions)
}

fn part2(input: &str) -> usize {
    let instructions = get_instructions(input, true, false);
    let transpose_area = area(instructions);
    let instructions = get_instructions(input, true, false);
    let normal_area = area(instructions);
    assert_eq!(transpose_area, normal_area);
    normal_area
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Edge {
    from: (isize, isize),
    to: (isize, isize),
}

impl Edge {
    fn intersects_y(&self, y: isize) -> bool {
        y >= self.from.1 && y <= self.to.1 || y >= self.to.1 && y <= self.from.1
    }
}

fn get_instructions(input: &str, part2: bool, transpose: bool) -> Vec<DigInstruction> {
    input
        .trim()
        .trim_matches('\n')
        .lines()
        .map(|line| line.parse::<ParsedDigInstruction>().unwrap())
        .map(|pdi| {
            if part2 {
                DigInstruction::part2_from(pdi)
            } else {
                DigInstruction::part1_from(pdi)
            }
        })
        .map(|mut i| {
            if transpose {
                i.direction = i.direction.transpose();
            }
            i
        })
        .collect()
}

fn get_edges(instructions: &[DigInstruction]) -> Vec<Edge> {
    let mut x = 0;
    let mut y = 0;
    instructions
        .iter()
        .map(|instruction| {
            let (rel_x, rel_y) = instruction.direction.rel();
            let new_x = x + rel_x * instruction.cells as isize;
            let new_y = y + rel_y * instruction.cells as isize;
            let e = Edge {
                from: (x, y),
                to: (new_x, new_y),
            };
            x = new_x;
            y = new_y;
            e
        })
        .collect()
}

fn get_vertical_edges(edges: &[Edge]) -> Vec<Edge> {
    edges
        .iter()
        .filter(|&e| e.to.0 == e.from.0)
        .cloned()
        .collect()
}

fn get_horizontal_edges(edges: &[Edge]) -> Vec<Edge> {
    edges
        .iter()
        .filter(|e| e.to.1 == e.from.1)
        .cloned()
        .collect()
}

fn area(instructions: Vec<DigInstruction>) -> usize {
    let edges = get_edges(&instructions);
    let vertical_edges = get_vertical_edges(&edges);
    let horizontal_edges = get_horizontal_edges(&edges);
    let min_y = vertical_edges
        .iter()
        .map(|e| e.from.1.min(e.to.1))
        .min()
        .unwrap();
    let max_y = vertical_edges
        .iter()
        .map(|e| e.from.1.max(e.to.1))
        .max()
        .unwrap();
    // let min_x = vertical_edges
    //     .iter()
    //     .map(|e| e.from.0.min(e.to.0))
    //     .min()
    //     .unwrap();
    // let max_x = vertical_edges
    //     .iter()
    //     .map(|e| e.from.0.max(e.to.0))
    //     .max()
    //     .unwrap();

    let perimeter: isize = instructions.iter().map(|i| i.cells as isize).sum();

    let interior_area: isize = (min_y..=max_y)
        .map(|y| scan_row(y, &vertical_edges, &horizontal_edges))
        .sum();
    (perimeter + interior_area) as usize
}

fn scan_row(y: isize, vertical_edges: &[Edge], horizontal_edges: &[Edge]) -> isize {
    let hedges: Vec<_> = horizontal_edges
        .iter()
        .filter(|e| e.intersects_y(y))
        .collect();
    let mut edges: Vec<_> = vertical_edges
        .iter()
        .filter(|e| e.intersects_y(y))
        .collect();
    edges.sort_unstable_by_key(|e| e.from.0);

    let mut prev = None;
    let mut inside = false;
    let mut total = 0;
    for e in edges {
        let Some(p) = prev.take() else {
            prev = Some(e);
            continue;
        };
        let e_vert = y == e.from.1 || y == e.to.1;
        let e_del = y - e.from.1 + y - e.to.1;
        let p_vert = y == p.from.1 || y == p.to.1;
        let p_del = y - p.from.1 + y - p.to.1;
        let inflex = e_del.is_positive() != p_del.is_positive();
        // println!("{p:?} {e:?}");
        if p_vert
            && e_vert
            && (hedges.contains(&&Edge {
                from: (e.from.0, y),
                to: (p.from.0, y),
            }) || hedges.contains(&&Edge {
                from: (p.from.0, y),
                to: (e.from.0, y),
            }))
        {
            if !inflex {
                inside = !inside;
            }
            // println!("skip");
            prev = Some(e);
            continue;
        }
        inside = !inside;
        if inside {
            total += e.from.0 - p.from.0 - 1;
        }
        // println!("{inside} {total}");
        prev = Some(e);
    }
    // println!("{total}");
    total
}

struct ParsedDigInstruction {
    direction: Direction,
    cells: usize,
    color: Color,
}

impl FromStr for ParsedDigInstruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut space_split = s.split(' ');
        let direction = space_split.next().unwrap().parse().unwrap();
        let cells = space_split.next().unwrap().parse().unwrap();
        let color = space_split.next().unwrap().parse().unwrap();
        Ok(Self {
            direction,
            cells,
            color,
        })
    }
}

struct DigInstruction {
    direction: Direction,
    cells: usize,
}

impl DigInstruction {
    fn part1_from(parsed: ParsedDigInstruction) -> Self {
        Self {
            direction: parsed.direction,
            cells: parsed.cells,
        }
    }

    fn part2_from(parsed: ParsedDigInstruction) -> Self {
        Self {
            direction: parsed.color.direction,
            cells: parsed.color.cells,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up = 1,
    Right = 2,
    Down = 3,
    Left = 4,
}

impl Direction {
    fn rel(&self) -> (isize, isize) {
        match self {
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
        }
    }

    fn transpose(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
            Self::Down => Self::Right,
        }
    }
}

impl From<isize> for Direction {
    fn from(value: isize) -> Self {
        match value {
            1 => Self::Up,
            2 => Self::Right,
            3 => Self::Down,
            4 => Self::Left,
            _ => panic!("invalid direction isize: {value}"),
        }
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => panic!("unknown direction {s:?}"),
        }
    }
}

#[derive(Debug, Clone)]
struct Color {
    direction: Direction,
    cells: usize,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("(#").unwrap();
        let s = s.strip_suffix(')').unwrap();
        let cells = usize::from_str_radix(&s[0..5], 16).unwrap();
        let direction = (isize::from_str_radix(&s[5..], 16).unwrap() + 1).into();
        Ok(Self { direction, cells })
    }
}

#[ignore]
#[test]
fn example1() {
    let input = "
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
    ";

    assert_eq!(area(get_instructions(input, false, false)), 62);
    assert_eq!(area(get_instructions(input, false, true)), 62);
}

#[ignore]
#[test]
fn example2() {
    let input = "
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
    ";

    assert_eq!(area(get_instructions(input, true, false)), 952408144115);
    assert_eq!(area(get_instructions(input, true, true)), 952408144115);
}

#[test]
fn test_scan_row() {
    let input = "
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
    ";
    let instructions = get_instructions(input, false, false);
    let edges = get_edges(&instructions);
    let vertical_edges = get_vertical_edges(&edges);
    let horizontal_edges = get_horizontal_edges(&edges);
    assert_eq!(scan_row(0, &vertical_edges, &horizontal_edges), 0);
    assert_eq!(scan_row(1, &vertical_edges, &horizontal_edges), 5);
    assert_eq!(scan_row(2, &vertical_edges, &horizontal_edges), 3);
    assert_eq!(scan_row(3, &vertical_edges, &horizontal_edges), 3);
    assert_eq!(scan_row(4, &vertical_edges, &horizontal_edges), 3);
    assert_eq!(scan_row(5, &vertical_edges, &horizontal_edges), 1);
    assert_eq!(scan_row(6, &vertical_edges, &horizontal_edges), 3);
    assert_eq!(scan_row(7, &vertical_edges, &horizontal_edges), 2);
    assert_eq!(scan_row(8, &vertical_edges, &horizontal_edges), 4);
    assert_eq!(scan_row(9, &vertical_edges, &horizontal_edges), 0);
}

#[test]
fn test_scan_row_careful1() {
    #[rustfmt::skip]
    let vertical_edges = vec![
        Edge{from: (0, 0), to: (0, 2)},
        Edge{from: (2, 0), to: (2, 1)},
        Edge{from: (3, 1), to: (3, 2)},
    ];
    #[rustfmt::skip]
    let horizontal_edges = vec![
        Edge {from: (0, 0), to: (2, 0)},
        Edge {from: (2, 1), to: (3, 1)},
        Edge {from: (0, 2), to: (3, 2)},
    ];
    assert_eq!(scan_row(0, &vertical_edges, &horizontal_edges), 0);
    assert_eq!(scan_row(1, &vertical_edges, &horizontal_edges), 1);
    assert_eq!(scan_row(2, &vertical_edges, &horizontal_edges), 0);
}

/*
#[test]
fn test_scan_row_careful3() {
    #[rustfmt::skip]
    let vertical_edges = vec![
        Edge{from: (0, 0), to: (0, 5)},
        Edge{from: (2, 0), to: (2, 1)},
        Edge{from: (4, 1), to: (4, 2)},
        Edge{from: (6, 2), to: (6, 1)},
        Edge{from: (7, 1), to: (7, 2)},
        Edge{from: (9, 2), to: (9, 1)},
        Edge{from: (10, 1), to: (10, 0)},
        Edge{from: (11, 0), to: (11, -1)},
        Edge{from: (12, 5), to: (12, -1)},
    ];
    assert_eq!(scan_row(0, &vertical_edges), 0);
    assert_eq!(scan_row(1, &vertical_edges), 2);
    assert_eq!(scan_row(2, &vertical_edges), 5);
}
*/

#[test]
fn test_scan_row_careful4() {
    #[rustfmt::skip]
    let vertical_edges = vec![
        Edge{from: (2, -1), to: (2, 1)},
        Edge{from: (4, 1), to: (4, 2)},
        Edge{from: (6, 2), to: (6, 1)},
        Edge{from: (7, 1), to: (7, 2)},
        Edge{from: (9, 2), to: (9, 1)},
        Edge{from: (10, 1), to: (10, 0)},
        Edge{from: (11, 0), to: (11, -1)},
    ];
    #[rustfmt::skip]
    let horizontal_edges = vec![
        Edge {from: (2, 1), to: (4, 1)},
        Edge {from: (4, 2), to: (6, 2)},
        Edge {from: (6, 1), to: (7, 1)},
        Edge {from: (7, 2), to: (9, 2)},
        Edge {from: (9, 1), to: (10, 1)},
        Edge {from: (10, 0), to: (11, 0)},
        Edge {from: (11, -1), to: (2, -1)},
    ];
    assert_eq!(scan_row(0, &vertical_edges, &horizontal_edges), 7);
    assert_eq!(scan_row(1, &vertical_edges, &horizontal_edges), 2);
}

use std::{
    collections::{HashSet, VecDeque},
    io::stdin,
    str::FromStr,
};

use strum::IntoEnumIterator;

fn main() {
    let s = std::io::read_to_string(stdin()).unwrap();
    let grid: Grid<Pipe> = s.parse().unwrap();
    let d = find_furthest_pipe_length(&grid);
    println!("Length = {d}");
    let e = find_enclosed_area(grid);
    println!("Enclosed = {e}");
}

fn find_start(grid: &Grid<Pipe>) -> Option<Coord> {
    grid.enumerate_coords()
        .find(|coord| matches!(grid.get(*coord), Some(Pipe::Start)))
}

fn find_adj_start(
    grid: &Grid<Pipe>,
    start: Coord,
) -> Option<((Coord, Direction), (Coord, Direction))> {
    let cs: Vec<(Coord, Direction)> = Direction::iter()
        .filter_map(|d| {
            let c = add_coords(d.relative_coord(), start);
            let Pipe::Section(d1, d2) = grid.get(c)? else {
                return None;
            };
            if d1.opposite() == d || d2.opposite() == d {
                Some((c, d.opposite()))
            } else {
                None
            }
        })
        .collect();
    let arr: [(Coord, Direction); 2] = cs.try_into().ok()?;
    Some(arr.into())
}

fn follow_pipe(
    grid: &Grid<Pipe>,
    c: Coord,
    from_direction: &Direction,
) -> Option<(Coord, Direction)> {
    let Pipe::Section(d1, d2) = grid.get(c)? else {
        return None;
    };
    if d1 == from_direction {
        let c = add_coords(c, d2.relative_coord());
        return Some((c, d2.opposite()));
    } else if d2 == from_direction {
        let c = add_coords(c, d1.relative_coord());
        return Some((c, d1.opposite()));
    } else {
        return None;
    }
}

fn find_furthest_pipe_length(grid: &Grid<Pipe>) -> usize {
    let start = find_start(grid).unwrap();
    let ((mut c1, mut d1), (mut c2, mut d2)) = find_adj_start(grid, start).unwrap();
    let mut count = 1;
    loop {
        if c1 == c2 {
            break;
        }
        (c1, d1) = follow_pipe(grid, c1, &d1).unwrap();
        (c2, d2) = follow_pipe(grid, c2, &d2).unwrap();
        count += 1;
    }
    count
}

fn find_enclosed_area(mut grid: Grid<Pipe>) -> usize {
    let mut l = VecDeque::new();
    let start = find_start(&grid).unwrap();
    l.push_front(start);
    println!("{grid}");
    let ((mut c1, mut d1), (mut c2, mut d2)) = find_adj_start(&grid, start).unwrap();
    let start = grid.get_mut(start).unwrap();
    *start = Pipe::Section(d1.opposite(), d2.opposite());
    loop {
        if c1 == c2 {
            l.push_front(c1);
            break;
        }
        l.push_front(c1);
        l.push_back(c2);
        (c1, d1) = follow_pipe(&grid, c1, &d1).unwrap();
        (c2, d2) = follow_pipe(&grid, c2, &d2).unwrap();
    }

    let directions: Vec<Direction> = (0..l.len())
        .map(|i| (l[i], l[(i + 1) % l.len()]))
        .map(|(c1, c2)| Direction::from_relative_coord(sub_coords(c1, c2)).unwrap())
        .collect();
    let winding: isize = (0..directions.len())
        .map(|i| (directions[i], directions[(i + 1) % directions.len()]))
        .map(|(d1, d2)| {
            if d1.right() == d2 {
                1
            } else if d1.left() == d2 {
                -1
            } else {
                0
            }
        })
        .sum();
    let side_func = if winding < 0 {
        println!("right winding");
        Direction::right
    } else if winding > 0 {
        println!("left winding");
        Direction::left
    } else {
        panic!("winding is zero");
    };
    let mut enclosed = HashSet::new();
    (0..l.len())
        .map(|i| (l[i], l[(i + 1) % l.len()]))
        .for_each(|(c1, c2)| {
            let d = Direction::from_relative_coord(sub_coords(c1, c2)).unwrap();
            let a = add_coords(c1, side_func(&d).relative_coord());
            if grid.get(a).is_some() && !l.contains(&a) {
                flood(&grid, l.as_slices(), a, &mut enclosed);
            }
            let b = add_coords(c2, side_func(&d).relative_coord());
            if grid.get(b).is_some() && !l.contains(&b) {
                flood(&grid, l.as_slices(), b, &mut enclosed);
            }
        });
    for c in &enclosed {
        let c = grid.get_mut(*c).unwrap();
        *c = Pipe::Section(Direction::North, Direction::North);
    }
    println!("{grid}");
    enclosed.len()
}

fn flood(
    grid: &Grid<Pipe>,
    pipe: (&[Coord], &[Coord]),
    coord: Coord,
    enclosed: &mut HashSet<Coord>,
) {
    let mut to_visit: Vec<Coord> = vec![coord];
    while let Some(c) = to_visit.pop() {
        if pipe.0.contains(&c) {
            continue;
        }
        if pipe.1.contains(&c) {
            continue;
        }
        enclosed.insert(c);
        for d in Direction::iter() {
            let t = add_coords(d.relative_coord(), c);
            if grid.get(t).is_none() {
                continue;
            }
            if enclosed.contains(&t) {
                continue;
            }
            to_visit.push(t);
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, strum::EnumIter)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn from_relative_coord(c: Coord) -> Option<Self> {
        match c {
            (0, -1) => Some(Self::North),
            (1, 0) => Some(Self::East),
            (0, 1) => Some(Self::South),
            (-1, 0) => Some(Self::West),
            _ => None,
        }
    }

    fn relative_coord(&self) -> Coord {
        match self {
            Self::North => (0, -1),
            Self::East => (1, 0),
            Self::South => (0, 1),
            Self::West => (-1, 0),
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }

    fn left(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }

    fn right(&self) -> Self {
        self.left().opposite()
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Pipe {
    Start,
    Ground,
    Section(Direction, Direction),
}

impl Pipe {
    fn from_char(c: char) -> Result<Self, MyError> {
        use Direction::*;
        match c {
            '|' => Ok(Self::Section(North, South)),
            '-' => Ok(Self::Section(East, West)),
            'L' => Ok(Self::Section(North, East)),
            'J' => Ok(Self::Section(North, West)),
            '7' => Ok(Self::Section(South, West)),
            'F' => Ok(Self::Section(South, East)),
            '.' => Ok(Self::Ground),
            'S' => Ok(Self::Start),
            c => Err(MyError::UnknownPipeChar(c)),
        }
    }

    fn as_char(&self) -> char {
        use Direction::*;
        match self {
            Self::Start => 'S',
            Self::Ground => '@',
            Self::Section(North, South) => '│',
            Self::Section(East, West) => '─',
            Self::Section(North, East) => '└',
            Self::Section(North, West) => '┘',
            Self::Section(South, West) => '┐',
            Self::Section(South, East) => '┌',
            Self::Section(_, _) => '#',
        }
    }
}

impl std::fmt::Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

type Coord = (isize, isize);

fn add_coords(a: Coord, b: Coord) -> Coord {
    (a.0 + b.0, a.1 + b.1)
}

fn sub_coords(a: Coord, b: Coord) -> Coord {
    (a.0 - b.0, a.1 - b.1)
}

struct Grid<T> {
    elements: Vec<T>,
    width: isize,
}

impl<T> std::fmt::Debug for Grid<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.height() {
            let index = (i * self.width) as usize..(i * self.width + self.width) as usize;
            f.write_fmt(format_args!("{:?}\n", &self.elements[index]))?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Grid<Pipe> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.height() {
            let index = (i * self.width) as usize..(i * self.width + self.width) as usize;
            f.write_str(&String::from_iter(
                self.elements[index].iter().map(|p| p.as_char()),
            ))?;
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Grid<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.height() {
            let index = (i * self.width) as usize..(i * self.width + self.width) as usize;
            f.write_str(&String::from_iter(&self.elements[index]))?;
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl<T> Grid<T> {
    fn calc_index(&self, coord: Coord) -> Option<isize> {
        if !(0..self.width()).contains(&coord.0) || !(0..self.height()).contains(&coord.1) {
            return None;
        }
        Some(self.width * coord.1 + coord.0)
    }

    pub fn get(&self, coord: Coord) -> Option<&T> {
        self.elements.get(self.calc_index(coord)? as usize)
    }

    pub fn get_mut(&mut self, coord: Coord) -> Option<&mut T> {
        let index = self.calc_index(coord)? as usize;
        self.elements.get_mut(index)
    }

    pub fn width(&self) -> isize {
        self.width
    }

    pub fn height(&self) -> isize {
        self.elements.len() as isize / self.width
    }

    pub fn enumerate_coords(&self) -> impl Iterator<Item = Coord> + '_ {
        (0..self.height()).flat_map(|y| (0..self.width()).map(move |x| (x, y)))
    }
}

impl FromStr for Grid<Pipe> {
    type Err = MyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !all_eq(&mut s.lines().map(|l| l.len())) {
            return Err(MyError::LinesAreNotSameLength);
        }
        let width = s.lines().next().unwrap().len() as isize;
        let elements = s
            .lines()
            .flat_map(|l| l.chars().map(|c| Pipe::from_char(c)))
            .collect::<Result<Vec<Pipe>, MyError>>()?;
        Ok(Self { elements, width })
    }
}

fn all_eq<I, T>(iter: &mut I) -> bool
where
    I: Iterator<Item = T>,
    T: PartialEq,
{
    let Some(first) = iter.next() else {
        return true;
    };
    iter.all(|e| first == e)
}

#[derive(Debug, thiserror::Error)]
enum MyError {
    #[error("lines are not the same length")]
    LinesAreNotSameLength,
    #[error("unknown pipe char {0}")]
    UnknownPipeChar(char),
}

#[cfg(test)]
mod tests {
    use crate::{
        find_adj_start, find_enclosed_area, find_furthest_pipe_length, find_start, Direction, Grid,
        Pipe,
    };

    fn clean_input(input: &str) -> &str {
        input.trim().trim_matches('\n')
    }

    #[test]
    fn simple_loop() {
        let input = "
.....
.S-7.
.|.|.
.L-J.
.....
        ";
        let grid: Grid<Pipe> = clean_input(input).parse().unwrap();
        println!("{grid:?}");
        let start = find_start(&grid).unwrap();
        assert_eq!(start, (1, 1));
        let (c1, c2) = find_adj_start(&grid, start).unwrap();
        assert!(c1.1 == Direction::West || c1.1 == Direction::North);
        assert!(c2.1 == Direction::West || c2.1 == Direction::North);
        assert!(c1.0 != c2.0);
        assert!(c1.0 == (1, 2) || c2.0 == (1, 2));
        assert!(c1.0 == (2, 1) || c2.0 == (2, 1));

        let d = find_furthest_pipe_length(&grid);
        assert_eq!(d, 4);
    }

    #[test]
    fn complex_loop() {
        let input = "
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
        ";
        let grid: Grid<Pipe> = clean_input(input).parse().unwrap();
        println!("{grid:?}");

        let d = find_furthest_pipe_length(&grid);
        assert_eq!(d, 8);
    }

    #[test]
    fn enclosed_loop() {
        let input = "
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
        ";

        let grid: Grid<Pipe> = clean_input(input).parse().unwrap();
        let area = find_enclosed_area(grid);
        assert_eq!(area, 4);
    }

    #[test]
    fn enclosed_loop_complex() {
        let input = "
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
        ";

        let grid: Grid<Pipe> = clean_input(input).parse().unwrap();
        let area = find_enclosed_area(grid);
        assert_eq!(area, 8);
    }

    #[test]
    fn enclosed_loop_complex2() {
        let input = "
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
        ";

        let grid: Grid<Pipe> = clean_input(input).parse().unwrap();
        let area = find_enclosed_area(grid);
        assert_eq!(area, 10);
    }
}

#![allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]

use std::{fmt::Write as _, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T> {
    data: Box<[T]>,
    width: isize,
}

impl<T> Grid<T> {
    #[must_use]
    pub fn new(data: Box<[T]>, width: isize) -> Self {
        let height = data.len() as isize / width;
        debug_assert_eq!(height * width, data.len() as isize);
        Self { data, width }
    }

    #[inline]
    #[must_use]
    pub const fn width(&self) -> isize {
        self.width
    }

    #[inline]
    #[must_use]
    pub const fn height(&self) -> isize {
        self.data.len() as isize / self.width
    }

    fn x_in_bounds(&self, x: isize) -> bool {
        (0..self.width()).contains(&x)
    }

    fn y_in_bounds(&self, y: isize) -> bool {
        (0..self.height()).contains(&y)
    }

    fn coord_to_index(&self, Vec2 { x, y }: Vec2) -> Option<usize> {
        if self.x_in_bounds(x) && self.y_in_bounds(y) {
            Some((y * self.width() + x) as usize)
        } else {
            None
        }
    }

    #[inline]
    pub fn get(&self, c: impl Into<Vec2>) -> Option<&T> {
        self.data.get(self.coord_to_index(c.into())?)
    }

    #[inline]
    pub fn get_mut(&mut self, c: impl Into<Vec2>) -> Option<&mut T> {
        let index = self.coord_to_index(c.into())?;
        self.data.get_mut(index)
    }

    /// Swaps the elements at `a` and `b`
    ///
    /// # Panics
    /// Panics if `a` or `b` is out of bounds
    pub fn swap(&mut self, a: impl Into<Vec2>, b: impl Into<Vec2>) {
        let a = self.coord_to_index(a.into()).expect("a is out of bounds");
        let b = self.coord_to_index(b.into()).expect("b is out of bounds");
        self.data.swap(a, b);
    }

    pub fn coords_iter(&self) -> impl Iterator<Item = Vec2> {
        let height = self.height();
        let width = self.width();
        (0..height).flat_map(move |y| (0..width).map(move |x| Vec2 { x, y }))
    }

    pub fn flat_iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    #[must_use]
    pub const fn iter(&self) -> GridIter<'_, T> {
        GridIter { grid: self, y: 0 }
    }
}

impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = GridRowIter<'a, T>;
    type IntoIter = GridIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Clone)]
pub struct GridIter<'a, T> {
    grid: &'a Grid<T>,
    y: isize,
}

impl<'a, T> Iterator for GridIter<'a, T> {
    type Item = GridRowIter<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.grid.y_in_bounds(self.y) {
            let out = GridRowIter {
                grid: self.grid,
                position: Vec2::new(0, self.y),
            };
            self.y += 1;
            Some(out)
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct GridRowIter<'a, T> {
    grid: &'a Grid<T>,
    position: Vec2,
}

impl<'a, T> Iterator for GridRowIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.grid.get(self.position)?;
        self.position.x += 1;
        Some(out)
    }
}

impl<T> std::fmt::Display for Grid<T>
where
    for<'a> &'a T: Into<char>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self {
            for cell in row {
                let c: char = cell.into();
                f.write_char(c)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl<T> FromStr for Grid<T>
where
    T: TryFrom<char>,
{
    type Err = GridParseError<T::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = None;
        if !s.lines().all(|l| {
            let w = l.chars().count() as isize;
            if let Some(expected_width) = width {
                expected_width == w
            } else {
                width = Some(w);
                true
            }
        }) {
            return Err(GridParseError::RowsDifferenWidth);
        }
        let width = width.ok_or(GridParseError::Empty)?;
        let data: Box<[T]> = s
            .lines()
            .flat_map(|l| l.chars())
            .map(|c| T::try_from(c))
            .collect::<Result<_, T::Error>>()?;
        let height = data.len() as isize / width;
        debug_assert_eq!(data.len() as isize, width * height);
        Ok(Self { data, width })
    }
}

#[derive(Debug)]
pub enum GridParseError<E> {
    Empty,
    RowsDifferenWidth,
    CharParse(E),
}

impl<E> From<E> for GridParseError<E> {
    fn from(err: E) -> Self {
        Self::CharParse(err)
    }
}

impl<E> std::error::Error for GridParseError<E> where E: std::error::Error {}

impl<E> std::fmt::Display for GridParseError<E>
where
    E: std::error::Error,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str("empty input"),
            Self::RowsDifferenWidth => f.write_str("rows different widths"),
            Self::CharParse(err) => f.write_fmt(format_args!("char parse: {err}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2 {
    pub x: isize,
    pub y: isize,
}

impl Vec2 {
    #[must_use]
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Mul<isize> for Vec2 {
    type Output = Self;

    fn mul(mut self, rhs: isize) -> Self::Output {
        self.x *= rhs;
        self.y *= rhs;
        self
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self
    }
}

impl std::ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> TryFrom<(T, T)> for Vec2
where
    T: TryInto<isize>,
{
    type Error = T::Error;

    fn try_from((x, y): (T, T)) -> Result<Self, Self::Error> {
        Ok(Self {
            x: x.try_into()?,
            y: y.try_into()?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    #[must_use]
    pub const fn rotate_clockwise(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

impl From<Direction> for Vec2 {
    fn from(d: Direction) -> Self {
        match d {
            Direction::North => Self { x: 0, y: -1 },
            Direction::East => Self { x: 1, y: 0 },
            Direction::South => Self { x: 0, y: 1 },
            Direction::West => Self { x: -1, y: 0 },
        }
    }
}

impl From<Direction> for u8 {
    fn from(d: Direction) -> Self {
        match d {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        }
    }
}

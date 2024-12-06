use std::str::FromStr;

#[derive(Clone)]
pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    pub fn new(data: Vec<T>, width: usize) -> Self {
        let height = data.len() / width;
        debug_assert_eq!(height * width, data.len());
        Self {
            data,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn coord_to_index(&self, Vec2 { x, y }: Vec2) -> Option<usize> {
        if (0..(self.width() as isize)).contains(&x) && (0..(self.height() as isize)).contains(&y) {
            Some(y as usize * self.width() + x as usize)
        } else {
            None
        }
    }

    pub fn get(&self, c: impl Into<Vec2>) -> Option<&T> {
        Some(self.data.get(self.coord_to_index(c.into())?).unwrap())
    }

    pub fn get_mut(&mut self, c: impl Into<Vec2>) -> Option<&mut T> {
        let index = self.coord_to_index(c.into())?;
        Some(self.data.get_mut(index).unwrap())
    }

    pub fn get_old(&self, c: (usize, usize)) -> Option<&T> {
        Some(
            self.data
                .get(self.coord_to_index(c.try_into().unwrap())?)
                .unwrap(),
        )
    }

    pub fn coords_iter(&self) -> impl Iterator<Item = (usize, usize)> {
        let height = self.height();
        let width = self.width();
        (0..height).flat_map(move |y| (0..width).map(move |x| (x, y)))
    }

    pub fn coords_iter_new(&self) -> impl Iterator<Item = Vec2> {
        let height = self.height() as isize;
        let width = self.width() as isize;
        (0..height).flat_map(move |y| (0..width).map(move |x| Vec2 { x, y }))
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }
}

impl<T> FromStr for Grid<T>
where
    T: TryFrom<char>,
{
    type Err = T::Error;

    fn from_str(s: &str) -> Result<Self, T::Error> {
        let width = s.lines().next().unwrap().len();
        let data: Vec<T> = s
            .lines()
            .flat_map(|l| l.chars())
            .map(|c| T::try_from(c))
            .collect::<Result<_, T::Error>>()?;
        let height = data.len() / width;
        debug_assert_eq!(data.len(), width * height);
        Ok(Self {
            data,
            width,
            height,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2 {
    pub x: isize,
    pub y: isize,
}

impl std::ops::Add for Vec2 {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}

impl<T> TryFrom<(T, T)> for Vec2
where
    isize: TryFrom<T>,
{
    type Error = <isize as TryFrom<T>>::Error;

    fn try_from((x, y): (T, T)) -> Result<Self, Self::Error> {
        Ok(Self {
            x: isize::try_from(x)?,
            y: isize::try_from(y)?,
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
    pub fn rotate_clockwise(self) -> Self {
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
            Direction::North => Vec2 { x: 0, y: -1 },
            Direction::East => Vec2 { x: 1, y: 0 },
            Direction::South => Vec2 { x: 0, y: 1 },
            Direction::West => Vec2 { x: -1, y: 0 },
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

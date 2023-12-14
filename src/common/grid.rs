use std::str::FromStr;

pub type Coord = (isize, isize);

pub fn add_coords(a: Coord, b: Coord) -> Coord {
    (a.0 + b.0, a.1 + b.1)
}

pub fn sub_coords(a: Coord, b: Coord) -> Coord {
    (a.0 - b.0, a.1 - b.1)
}

#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    elements: Vec<T>,
    width: isize,
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

    /// Produces iterator over coords row by row
    pub fn enumerate_coords(&self) -> impl Iterator<Item = Coord> + '_ {
        (0..self.height()).flat_map(|y| (0..self.width()).map(move |x| (x, y)))
    }

    #[allow(dead_code)]
    pub fn insert_row(
        &mut self,
        index: isize,
        elements: impl Iterator<Item = T>,
    ) -> Result<(), GridError> {
        let els: Vec<_> = elements.take(self.width as usize).collect();
        if els.len() as isize != self.width {
            return Err(GridError::WrongElementsLength);
        }
        let index = (index * self.width) as usize;
        for (i, el) in els.into_iter().enumerate() {
            self.elements.insert(index + i, el);
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub fn insert_column(
        &mut self,
        index: isize,
        elements: impl Iterator<Item = T>,
    ) -> Result<(), GridError> {
        let els: Vec<_> = elements.take(self.height() as usize).collect();
        if els.len() as isize != self.height() {
            return Err(GridError::WrongElementsLength);
        }
        self.width += 1;
        for (i, el) in els.into_iter().enumerate() {
            let index = index as usize + (i * self.width as usize);
            self.elements.insert(index, el);
        }
        Ok(())
    }
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

impl<T> std::fmt::Display for Grid<T>
where
    for<'a> &'a T: Into<char>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.height() {
            let index = (i * self.width) as usize..(i * self.width + self.width) as usize;
            f.write_str(&String::from_iter(
                self.elements[index].iter().map(|x| x.into()),
            ))?;
            f.write_str("\n")?;
        }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GridError {
    #[allow(dead_code)]
    #[error("elements do not match expected length")]
    WrongElementsLength,
}

impl<T> FromStr for Grid<T>
where
    T: TryFrom<char>,
{
    type Err = GridParseError<T::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !super::all_eq(&mut s.lines().map(|l| l.len())) {
            return Err(GridParseError::LinesDifferentLength);
        }
        let width = s.lines().next().unwrap().len() as isize;
        let elements = s
            .lines()
            .flat_map(|l| l.chars().map(|c| Ok(T::try_from(c)?)))
            .collect::<Result<Vec<T>, Self::Err>>()?;
        Ok(Self { elements, width })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GridParseError<E> {
    #[error("lines are not the same length")]
    LinesDifferentLength,
    #[error("convert char: {0}")]
    ConvertChar(#[from] E),
}

use std::str::FromStr;

pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn coord_to_index(&self, (x, y): (usize, usize)) -> Option<usize> {
        if (0..self.width()).contains(&x) && (0..self.height()).contains(&y) {
            Some(y * self.width() + x)
        } else {
            None
        }
    }

    pub fn get(&self, c: (usize, usize)) -> Option<&T> {
        Some(self.data.get(self.coord_to_index(c)?).unwrap())
    }

    pub fn coords_iter(&self) -> impl Iterator<Item = (usize, usize)> {
        let height = self.height();
        let width = self.width();
        (0..height).flat_map(move |y| (0..width).map(move |x| (x, y)))
    }
}

impl FromStr for Grid<char> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().unwrap().len();
        let data: Vec<char> = s.lines().flat_map(|l| l.chars()).collect();
        let height = data.len() / width;
        debug_assert_eq!(data.len(), width * height);
        Ok(Self {
            data,
            width,
            height,
        })
    }
}

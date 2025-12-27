use std::{
    fmt::Write as _,
    ops::{Deref, DerefMut, Index, IndexMut},
};

use crate::magic_iter::MagicIterVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn area(&self, other: &Self) -> usize {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }

    pub fn rect_contains(&self, other: &Self, needle: &Self) -> bool {
        let min_x = self.x.min(other.x);
        let max_x = self.x.max(other.x);
        let min_y = self.y.min(other.y);
        let max_y = self.y.max(other.y);
        min_x <= needle.x && needle.x <= max_x && min_y <= needle.y && needle.y <= max_y
    }

    pub fn rect_iter(&self, other: &Self) -> impl Iterator<Item = Self> {
        let min_x = self.x.min(other.x);
        let max_x = self.x.max(other.x);
        let min_y = self.y.min(other.y);
        let max_y = self.y.max(other.y);
        (min_x..=max_x).flat_map(move |x| (min_y..=max_y).map(move |y| Self { x, y }))
    }

    pub fn rect_edges_iter(&self, other: &Self) -> impl Iterator<Item = Self> {
        let min_x = self.x.min(other.x);
        let max_x = self.x.max(other.x);
        let min_y = self.y.min(other.y);
        let max_y = self.y.max(other.y);
        let top = (min_x..=max_x).map(move |x| Self { x, y: min_y });
        let bottom = (min_x..=max_x).map(move |x| Self { x, y: max_y });
        let left = (min_y..=max_y).map(move |y| Self { x: min_x, y });
        let right = (min_y..=max_y).map(move |y| Self { x: max_x, y });
        top.chain(bottom).chain(left).chain(right)
    }

    pub fn adjacent(&self) -> impl Iterator<Item = Self> {
        let Self { x, y } = *self;
        [
            x.checked_sub(1).map(|x| Self { x, y }),
            x.checked_add(1).map(|x| Self { x, y }),
            y.checked_sub(1).map(|y| Self { x, y }),
            y.checked_add(1).map(|y| Self { x, y }),
        ]
        .into_iter()
        .flatten()
    }
}

#[derive(Clone)]
pub struct Grid<T> {
    cells: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Copy> Grid<T> {
    pub fn new_fill(value: T, width: usize, height: usize) -> Self {
        Self {
            cells: vec![value; width * height],
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

    pub fn get_point(&self, Point { x, y }: Point) -> Option<T> {
        if x < self.width && y < self.height {
            Some(self.cells[x + y * self.width])
        } else {
            None
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<T> {
        self.get_point(Point { x, y })
    }

    /// # Safety
    /// Point must be in bounds
    pub unsafe fn get_unchecked(&self, Point { x, y }: Point) -> T {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);
        unsafe { *self.cells.get_unchecked(x + y * self.width) }
    }

    pub fn set_point(&mut self, Point { x, y }: Point, v: T) {
        if x < self.width && y < self.height {
            self.cells[x + y * self.width] = v;
        } else {
            panic!("out of bounds {x}, {y}")
        }
    }

    pub fn set(&mut self, x: usize, y: usize, v: T) {
        self.set_point(Point { x, y }, v);
    }

    pub fn set_unchecked(&mut self, Point { x, y }: Point, v: T) {
        *unsafe { self.cells.get_unchecked_mut(x + y * self.width) } = v;
    }

    pub fn set_rect(&mut self, start: Point, end: Point, v: T) {
        for p in start.rect_iter(&end) {
            self.set_point(p, v);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = T> {
        (0..self.height).flat_map(move |y| {
            (0..self.width).map(move |x| unsafe { self.get_unchecked(Point { x, y }) })
        })
    }

    pub fn swap_rows(&mut self, i: usize, j: usize) {
        assert!(i < self.height);
        assert!(j < self.height);
        if i != j {
            for x in 0..self.width {
                self.cells.swap(x + i * self.width, x + j * self.width);
            }
        }
    }

    pub fn swap_columns(&mut self, i: usize, j: usize) {
        assert!(i < self.width);
        assert!(j < self.width);
        if i != j {
            for y in 0..self.height {
                self.cells.swap(i + y * self.width, j + y * self.width);
            }
        }
    }

    #[must_use]
    pub fn transpose(&self) -> Self {
        let mut out: Self = self.clone();
        std::mem::swap(&mut out.width, &mut out.height);
        for y in 0..self.height {
            for x in 0..self.width {
                out.set(y, x, self.get(x, y).unwrap());
            }
        }
        out
    }

    pub fn get_col(&self, i: usize) -> impl Iterator<Item = T> {
        (0..self.height).map(move |y| self.get(i, y).unwrap())
    }

    pub fn get_row(&self, y: usize) -> &[T] {
        assert!(y < self.height);
        &self.cells[y * self.width..(y + 1) * self.width]
    }

    pub fn add_row(&mut self, row: &[T]) {
        assert_eq!(row.len(), self.width);
        self.cells.extend_from_slice(row);
        self.height += 1;
    }

    pub fn remove_last_row(&mut self) {
        assert!(self.height > 0);
        self.height -= 1;
        self.cells.truncate(self.width * self.height);
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        if x < self.width && y < self.height {
            &self.cells[x + y * self.width]
        } else {
            panic!("out of bounds")
        }
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        if x < self.width && y < self.height {
            &mut self.cells[x + y * self.width]
        } else {
            panic!("out of bounds")
        }
    }
}

impl<T: std::fmt::Debug + Copy> Grid<T> {
    pub fn display(&self) -> String {
        let mut out = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                write!(&mut out, "{:>10?}", self.get_point(Point { x, y }).unwrap()).unwrap();
            }
            out.push('\n');
        }
        out
    }
}

impl Grid<f32> {
    pub fn guassian_elimination(&mut self) {
        let mut h = 0usize;
        let mut k = 0usize;
        while h < self.height && k < self.width {
            let i_max = (h..self.height)
                .max_by(|a, b| f32::total_cmp(&self[(k, *a)].abs(), &self[(k, *b)].abs()))
                .unwrap();
            if self[(k, i_max)] == 0.0 {
                k += 1;
                continue;
            }
            self.swap_rows(h, i_max);
            for i in h + 1..self.height {
                let f = self[(k, i)] / self[(k, h)];
                self[(k, i)] = 0.0;
                for j in k + 1..self.width {
                    self[(j, i)] -= self[(j, h)] * f;
                }
            }
            h += 1;
            k += 1;
        }
    }

    pub fn reduced_row_echelon(&mut self) {
        let n = self.width.min(self.height);
        for i in 0..n {
            let t = self[(i, i)];
            if t.abs() < 0.01 {
                continue;
            }
            for j in 0..self.width {
                self[(j, i)] /= t;
            }
        }
    }
}

#[derive(Clone)]
pub struct AugmentedMatrix<T>(pub Grid<T>);

impl Deref for AugmentedMatrix<isize> {
    type Target = Grid<isize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AugmentedMatrix<isize> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AugmentedMatrix<isize> {
    pub fn bareiss(&mut self) {
        let m = self.height;
        let n = self.width;
        let mut prev = 1;
        for k in 0..(n - 1).min(m - 1) {
            if self.0[(k, k)] == 0 {
                for i in (k + 1)..m {
                    if self[(k, i)] != 0 {
                        self.swap_rows(k, i);
                        break;
                    }
                }
            }
            let pivot = self[(k, k)];
            for i in (k + 1)..m {
                for j in (k + 1)..n {
                    let numerator = self[(j, i)] * pivot - self[(k, i)] * self[(j, k)];
                    let divisor = prev;
                    let Some(value) = numerator.checked_div(divisor) else {
                        return;
                    };
                    debug_assert_eq!(numerator % divisor, 0);
                    self[(j, i)] = value;
                }
            }
            for i in (k + 1)..m {
                self[(k, i)] = 0;
            }
            prev = pivot;
        }
        self.simplify_row_multiples();
    }

    pub fn simplify_row_multiples(&mut self) {
        for i in 0..self.height {
            let mut common = self[(0, i)];
            let mut j = 1;
            while j < self.width && common != 1 {
                common = num::integer::gcd(common, self[(j, i)]);
                j += 1;
            }
            if common != 1 && common != 0 {
                for j in 0..self.width {
                    self[(j, i)] /= common;
                }
            }
        }
    }

    /// Calculate the rank of the matrix, assumes the matrix is in row echelon form
    pub fn rank(&self) -> usize {
        let mut rank = 0;
        for i in 0..self.height {
            if self.get_row(i).iter().any(|x| *x != 0) {
                rank += 1;
            }
        }
        rank
    }

    pub fn remove_trailing_zero_rows(&mut self) {
        while self.0.get_row(self.0.height() - 1)[..self.0.width() - 1]
            .iter()
            .all(|x| *x == 0)
        {
            self.0.remove_last_row();
        }
    }

    #[expect(clippy::needless_range_loop)]
    pub fn check_solution(&self, v: &[isize]) -> bool {
        assert_eq!(v.len(), self.0.width - 1);
        for i in 0..self.0.height {
            let mut sum = 0;
            for j in 0..self.0.width - 1 {
                sum += self.0[(j, i)] * v[j];
            }
            if sum != self.0[(self.0.width - 1, i)] {
                return false;
            }
        }
        true
    }

    /// Must be called with the matrix in row echelon form
    #[expect(clippy::needless_range_loop)]
    pub fn solve(&self) -> Option<Vec<isize>> {
        if self.rank() < self.0.width - 1 {
            return None;
        }
        let mut answer = vec![None; self.0.width - 1];
        for i in (0..self.0.width - 1).rev() {
            let denominator = self.0[(i, i)];
            if denominator == 0 {
                return None;
            }
            let mut sum = 0;
            for j in i + 1..self.0.width - 1 {
                sum += answer[j]? * self.0[(j, i)];
            }
            let numerator = self.0[(self.0.width - 1, i)] - sum;
            if numerator % denominator != 0 {
                return None;
            }
            answer[i] = Some(numerator / denominator);
        }
        answer.into_iter().collect()
    }

    /// Must be called with the matrix in row echelon form
    pub fn solve_with_unknowns(&self, mut unknowns: &[usize]) -> Option<Vec<isize>> {
        let mut matrix = self.clone();
        for i in 0..matrix.width - 1 {
            if i >= matrix.height || matrix[(i, i)] == 0 {
                let mut v = vec![0; self.width];
                v[self.width - 1] = *unknowns.first()? as isize;
                unknowns = &unknowns[1..];
                v[i] = 1;
                matrix.add_row(&v);
                matrix.bareiss();
            }
        }
        assert!(unknowns.is_empty());
        matrix.solve()
    }

    /// Must be called with the matrix in row echelon form
    pub fn solve_indeterminate(&self) -> impl Iterator<Item = Vec<isize>> {
        let missing = (self.width - 1).saturating_sub(self.rank());
        MagicIterVec::new(missing)
            .filter_map(move |missing_values| self.solve_with_unknowns(&missing_values))
    }
}

#[test]
fn test_bareiss_solve() {
    let mut grid = Grid::new_fill(0isize, 4, 0);
    grid.add_row(&[1, 2, -1, 7]);
    grid.add_row(&[2, -3, -4, -3]);
    grid.add_row(&[1, 1, 1, 0]);
    let mut augmented_matrix = AugmentedMatrix(grid);
    let initial = augmented_matrix.clone();
    augmented_matrix.bareiss();
    println!("{}", augmented_matrix.0.display());
    assert_eq!(augmented_matrix.rank(), 3);
    let answer = augmented_matrix.solve().unwrap();
    assert!(augmented_matrix.check_solution(&answer));
    assert!(initial.check_solution(&answer));
}

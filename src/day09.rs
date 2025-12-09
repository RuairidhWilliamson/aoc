use kdam::par_tqdm;
use rayon::iter::{IntoParallelIterator, ParallelIterator as _};

pub fn part1(input: &str) -> usize {
    let points: Vec<Point> = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect();
    points
        .iter()
        .enumerate()
        .flat_map(|(i, a)| points[..i].iter().map(|b| a.area(b)))
        .max()
        .unwrap()
}

pub fn part2(input: &str) -> usize {
    let points: Vec<Point> = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect();

    let width = points.iter().map(|Point { x, y: _ }| *x).max().unwrap() + 1;
    let height = points.iter().map(|Point { x: _, y }| *y).max().unwrap() + 1;
    let mut grid = Grid::new_fill(0, width, height);
    for i in 0..points.len() {
        let a = points[i];
        let b = points[(i + 1) % points.len()];
        grid.set_rect(a, b, 1);
    }

    for p in &points {
        grid.set(*p, 2);
    }

    grid.fill_inner(3);

    let points_pairs: Vec<(Point, Point)> = points
        .iter()
        .enumerate()
        .flat_map(|(i, a)| points[..i].iter().map(move |b| (*a, *b)))
        .collect();

    // Remove obviously wrong rects
    let points_pairs: Vec<_> = par_tqdm!(points_pairs.into_par_iter())
        .filter(|(a, b)| {
            a.rect_edges_iter(b)
                .all(|p| unsafe { grid.get_unchecked(p) } != 0)
        })
        .collect();

    // Correctly filter the remaining rects properly
    par_tqdm!(points_pairs.into_par_iter())
        .filter(|(a, b)| {
            a.rect_iter(b)
                .all(|p| unsafe { grid.get_unchecked(p) } != 0)
        })
        .map(|(a, b)| a.area(&b))
        .max()
        .unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn area(&self, other: &Self) -> usize {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }

    fn rect_iter(&self, other: &Self) -> impl Iterator<Item = Point> {
        let min_x = self.x.min(other.x);
        let max_x = self.x.max(other.x);
        let min_y = self.y.min(other.y);
        let max_y = self.y.max(other.y);
        (min_x..=max_x).flat_map(move |x| (min_y..=max_y).map(move |y| Point { x, y }))
    }

    fn rect_edges_iter(&self, other: &Self) -> impl Iterator<Item = Point> {
        let min_x = self.x.min(other.x);
        let max_x = self.x.max(other.x);
        let min_y = self.y.min(other.y);
        let max_y = self.y.max(other.y);
        let top = (min_x..=max_x).map(move |x| Point { x, y: min_y });
        let bottom = (min_x..=max_x).map(move |x| Point { x, y: max_y });
        let left = (min_y..=max_y).map(move |y| Point { x: min_x, y });
        let right = (min_y..=max_y).map(move |y| Point { x: max_x, y });
        top.chain(bottom).chain(left).chain(right)
    }
}

struct Grid {
    cells: Vec<u8>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new_fill(value: u8, width: usize, height: usize) -> Self {
        Self {
            cells: vec![value; width * height],
            width,
            height,
        }
    }

    fn get(&self, Point { x, y }: Point) -> Option<u8> {
        if x < self.width && y < self.height {
            Some(self.cells[x + y * self.width])
        } else {
            None
        }
    }

    unsafe fn get_unchecked(&self, Point { x, y }: Point) -> u8 {
        unsafe { *self.cells.get_unchecked(x + y * self.width) }
    }

    fn set(&mut self, Point { x, y }: Point, v: u8) {
        if x < self.width && y < self.height {
            self.cells[x + y * self.width] = v;
        } else {
            panic!("out of bounds {x}, {y}")
        }
    }

    fn set_rect(&mut self, start: Point, end: Point, v: u8) {
        for p in start.rect_iter(&end) {
            self.set(p, v);
        }
    }

    fn fill_inner(&mut self, set_value: u8) {
        for y in 0..self.height {
            let mut inside = false;
            let mut edge = false;
            for x in 0..self.width {
                let v = self.get(Point { x, y }).unwrap();
                match v {
                    2 => {
                        edge = !edge;
                        if !edge {
                            inside = !inside;
                        }
                    }
                    1 if !edge => {
                        inside = !inside;
                    }
                    0 if inside && !edge => {
                        self.set(Point { x, y }, set_value);
                    }
                    _ => {}
                }
            }
        }
    }

    #[expect(dead_code)]
    fn display(&self) -> String {
        let mut out = String::with_capacity(self.width + (self.height + 1));
        for y in 0..self.height {
            for x in 0..self.width {
                out.push((b'0' + unsafe { self.get_unchecked(Point { x, y }) }) as char);
            }
            out.push('\n');
        }
        out
    }
}

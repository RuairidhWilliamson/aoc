use std::{
    collections::HashSet,
    time::{Duration, Instant},
};

use crate::{
    ascii_grid::AsciiGrid,
    grid::{Grid, Point},
};

pub fn part1(input: &str) -> usize {
    let mut sections: Vec<_> = input.split("\n\n").collect();
    let areas = sections.pop().unwrap();
    let areas: Vec<ProblemArea> = areas
        .lines()
        .map(|line| {
            let (size, counts) = line.split_once(": ").unwrap();
            let (width, height) = size.split_once('x').unwrap();
            let width = width.parse().unwrap();
            let height = height.parse().unwrap();
            let counts = counts
                .split(' ')
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            ProblemArea {
                width,
                height,
                counts,
            }
        })
        .collect();
    let mut shapes = Vec::new();
    for section in sections {
        let index = section.find('\n').unwrap();
        let shape = Shape(AsciiGrid::new(&section.as_bytes()[index + 1..]));
        assert_eq!(shape.0.width(), 3);
        assert_eq!(shape.0.height(), 3);
        shapes.push(shape);
    }
    areas.iter().filter(|a| a.solve(&shapes)).count()
}

pub fn part2(_input: &str) -> usize {
    0
}

struct ProblemArea {
    width: usize,
    height: usize,
    counts: Vec<usize>,
}

impl ProblemArea {
    fn solve(&self, shapes: &[Shape]) -> bool {
        let mut counts = self.counts.clone();
        let grid = Grid::new_fill(Cell::Vacant, self.width, self.height);
        let mut solver = Solver {
            grid,
            counts: &mut counts,
            shapes,
            start_time: Instant::now(),
        };

        solver.visit(None)
    }
}

struct Solver<'a> {
    grid: Grid<Cell>,
    counts: &'a mut [usize],
    shapes: &'a [Shape<'a>],
    start_time: Instant,
}

impl Solver<'_> {
    fn vacant_area(&self) -> usize {
        self.grid.iter().filter(|c| *c == Cell::Vacant).count()
    }

    fn min_shape_area(&self) -> usize {
        self.counts
            .iter()
            .enumerate()
            .filter(|(_, count)| **count > 0)
            .map(|(i, _)| self.shapes[i].area())
            .min()
            .unwrap_or_default()
    }

    fn total_area_of_shapes_to_add(&self) -> usize {
        self.counts
            .iter()
            .enumerate()
            .map(|(i, count)| count * self.shapes[i].area())
            .sum::<usize>()
    }

    fn place(&mut self, placement: &Placement) -> Result<(), ()> {
        let shape = &self.shapes[placement.id];
        if !(0..shape.0.height()).all(|y| {
            (0..shape.0.width()).all(|x| {
                shape.0.get(x, y).unwrap() == b'.'
                    || placement
                        .get_point(x as isize, y as isize)
                        .is_some_and(|p| self.grid.get_point(p) == Some(Cell::Vacant))
            })
        }) {
            return Err(());
        }
        self.counts[placement.id] -= 1;
        (0..shape.0.height()).for_each(|y| {
            (0..shape.0.width()).for_each(|x| {
                if shape.0.get(x, y).unwrap() == b'#' {
                    self.grid.set_point(
                        placement.get_point(x as isize, y as isize).unwrap(),
                        Cell::Occupied,
                    );
                }
            });
        });
        Ok(())
    }

    fn unplace(&mut self, placement: &Placement) {
        let shape = &self.shapes[placement.id];
        self.counts[placement.id] += 1;
        (0..shape.0.height()).for_each(|y| {
            (0..shape.0.width()).for_each(|x| {
                if shape.0.get(x, y).unwrap() == b'#' {
                    self.grid.set_point(
                        placement.get_point(x as isize, y as isize).unwrap(),
                        Cell::Vacant,
                    );
                }
            });
        });
    }

    fn can_potentially_fit_all_shapes(&self) -> bool {
        let mut unoccupiable_region_area = 0;
        let mut visited = HashSet::new();
        let mut to_visit = Vec::new();
        for y in 0..self.grid.height() {
            for x in 0..self.grid.width() {
                debug_assert!(to_visit.is_empty());
                let point = Point { x, y };
                if self.grid.get_point(point).unwrap() != Cell::Vacant {
                    continue;
                }
                if !visited.insert(point) {
                    continue;
                }
                to_visit.push(point);
                let mut connected_count = 0;
                while let Some(point) = to_visit.pop() {
                    for a in point.adjacent() {
                        if self.grid.get_point(a) != Some(Cell::Vacant) {
                            continue;
                        }
                        if visited.insert(a) {
                            connected_count += 1;
                            to_visit.push(a);
                        }
                    }
                }
                if connected_count < self.min_shape_area() {
                    unoccupiable_region_area += connected_count;
                }
            }
        }
        self.vacant_area() - unoccupiable_region_area >= self.total_area_of_shapes_to_add()
    }

    fn visit(&mut self, last_placement: Option<&Placement>) -> bool {
        if !self.can_potentially_fit_all_shapes() {
            return false;
        }
        let last_x = last_placement.map(|l| l.x).unwrap_or_default();
        let last_y = last_placement.map(|l| l.y).unwrap_or_default();
        if self.counts.iter().all(|c| *c == 0) {
            return true;
        }
        if self.start_time.elapsed() > Duration::from_secs(60) {
            return false;
        }
        for i in 0..self.shapes.len() {
            if self.counts[i] == 0 {
                continue;
            }
            for offset_x in -3..=3 {
                let Some(x) = last_x.checked_add_signed(offset_x) else {
                    continue;
                };
                for offset_y in -3..=3 {
                    let Some(y) = last_y.checked_add_signed(offset_y) else {
                        continue;
                    };
                    for rotate in 0..4 {
                        for flip in [false, true] {
                            let placement = Placement {
                                id: i,
                                x,
                                y,
                                rotate,
                                flip,
                            };
                            if self.place(&placement).is_err() {
                                continue;
                            }
                            if self.visit(Some(&placement)) {
                                return true;
                            }
                            self.unplace(&placement);
                        }
                    }
                }
            }
        }
        false
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Vacant,
    Occupied,
}

struct Shape<'a>(AsciiGrid<'a>);

impl Shape<'_> {
    fn area(&self) -> usize {
        self.0.iter().filter(|b| *b == b'#').count()
    }
}

struct Placement {
    id: usize,
    x: usize,
    y: usize,
    rotate: usize,
    flip: bool,
}

impl Placement {
    fn get_point(&self, x: isize, y: isize) -> Option<Point> {
        let (x, mut y) = match self.rotate {
            0 => (x, y),
            1 => (y, -x),
            2 => (-x, -y),
            3 => (-y, x),
            _ => panic!(),
        };
        if self.flip {
            y = -y;
        }
        Some(Point {
            x: self.x.checked_add_signed(x)?,
            y: self.y.checked_add_signed(y)?,
        })
    }
}

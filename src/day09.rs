use rayon::iter::{
    IndexedParallelIterator as _, IntoParallelRefIterator as _, ParallelBridge as _,
    ParallelIterator as _,
};

use crate::{
    env_is_enabled,
    grid::{Grid, Point},
};

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
        grid.set_point(*p, 2);
    }

    fill_inner(&mut grid, 3);

    let predicate = |(a, b): &(Point, Point)| {
        // Remove obviously wrong rects
        let obvious_check = a
            .rect_edges_iter(b)
            .all(|p| unsafe { grid.get_unchecked(p) } != 0);
        obvious_check
            && points
                .iter()
                .filter(|p| a.rect_contains(b, p))
                .flat_map(|Point { x, y }| {
                    [
                        Point { x: x - 1, y: y - 1 },
                        Point { x: x + 1, y: y - 1 },
                        Point { x: x + 1, y: y + 1 },
                        Point { x: x - 1, y: y + 1 },
                    ]
                })
                .all(|p| !a.rect_contains(b, &p) || grid.get_point(p) != Some(0))
    };
    if env_is_enabled("NO_RAYON") {
        points
            .iter()
            .enumerate()
            .flat_map(|(i, a)| points[..i].iter().map(move |b| (*a, *b)))
            .filter(predicate)
            .map(|(a, b)| a.area(&b))
            .max()
            .unwrap()
    } else {
        points
            .par_iter()
            .enumerate()
            .flat_map(|(i, a)| points[..i].par_iter().map(move |b| (*a, *b)))
            .filter(predicate)
            .map(|(a, b)| a.area(&b))
            .max()
            .unwrap()
    }
}

pub fn fill_inner(grid: &mut Grid<u8>, set_value: u8) {
    let iter = grid.get_all_rows_mut();
    let fill_row = |row: &mut [u8]| {
        let mut inside = false;
        let mut edge = false;
        for v in row {
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
                    *v = set_value;
                }
                _ => {}
            }
        }
    };
    if env_is_enabled("NO_RAYON") {
        iter.for_each(fill_row);
    } else {
        iter.par_bridge().for_each(fill_row);
    }
}

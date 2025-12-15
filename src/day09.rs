use crate::grid::{Grid, Point};

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

    points
        .iter()
        .enumerate()
        .flat_map(|(i, a)| points[..i].iter().map(move |b| (*a, *b)))
        .filter(|(a, b)| {
            // Remove obviously wrong rects
            a.rect_edges_iter(b)
                .all(|p| unsafe { grid.get_unchecked(p) } != 0)
        })
        .filter(|(a, b)| {
            points
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
        })
        .map(|(a, b)| a.area(&b))
        .max()
        .unwrap()
}

pub fn fill_inner(grid: &mut Grid<u8>, set_value: u8) {
    for y in 0..grid.height() {
        let mut inside = false;
        let mut edge = false;
        for x in 0..grid.width() {
            let v = unsafe { grid.get_unchecked(Point { x, y }) };
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
                    grid.set_unchecked(Point { x, y }, set_value);
                }
                _ => {}
            }
        }
    }
}

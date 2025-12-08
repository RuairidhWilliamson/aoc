use std::collections::HashSet;

pub fn part1(input: &str) -> usize {
    let points: Vec<Point> = input.lines().map(|line| Point::from_line(line)).collect();
    let mut edges: Vec<(Point, Point, usize)> = points
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            points[..i]
                .iter()
                .map(move |b| (*a, *b, a.l2_norm_square(b)))
        })
        .collect();
    debug_assert_eq!(edges.len(), (points.len() * (points.len() - 1)) / 2);
    edges.sort_by_key(|(_, _, l)| *l);
    let n = if points.len() < 1000 { 10 } else { 1000 };
    let edges = &edges[..n];
    let mut circuits: Vec<HashSet<Point>> = Vec::new();
    for (a, b, _) in edges {
        let a_index = find_circuit_containing_point(a, &circuits);
        let b_index = find_circuit_containing_point(b, &circuits);
        match (a_index, b_index) {
            (Some(i), Some(j)) if i == j => {}
            (Some(i), Some(j)) => {
                let [circuit_i, circuit_j] = circuits.get_disjoint_mut([i, j]).unwrap();
                for c in &*circuit_j {
                    circuit_i.insert(*c);
                }
                circuits.swap_remove(j);
            }
            (None, Some(i)) => {
                circuits[i].insert(*a);
            }
            (Some(i), None) => {
                circuits[i].insert(*b);
            }
            (None, None) => {
                let mut h = HashSet::new();
                h.insert(*a);
                h.insert(*b);
                circuits.push(h);
            }
        }
    }
    circuits.sort_by_key(|c| c.len());
    assert!(circuits.len() >= 3);
    circuits[circuits.len() - 3..]
        .iter()
        .map(|circuit| circuit.len())
        .product()
}

pub fn part2(input: &str) -> usize {
    let points: Vec<Point> = input.lines().map(|line| Point::from_line(line)).collect();
    let mut edges: Vec<(Point, Point, usize)> = points
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            points[..i]
                .iter()
                .map(move |b| (*a, *b, a.l2_norm_square(b)))
        })
        .collect();
    debug_assert_eq!(edges.len(), (points.len() * (points.len() - 1)) / 2);
    edges.sort_by_key(|(_, _, l)| *l);
    let mut circuits: Vec<HashSet<Point>> = Vec::new();
    for (a, b, _) in &edges {
        let a_index = find_circuit_containing_point(a, &circuits);
        let b_index = find_circuit_containing_point(b, &circuits);
        match (a_index, b_index) {
            (Some(i), Some(j)) if i == j => {}
            (Some(i), Some(j)) => {
                let [circuit_i, circuit_j] = circuits.get_disjoint_mut([i, j]).unwrap();
                for c in &*circuit_j {
                    circuit_i.insert(*c);
                }
                circuits.swap_remove(j);
            }
            (None, Some(i)) => {
                circuits[i].insert(*a);
            }
            (Some(i), None) => {
                circuits[i].insert(*b);
            }
            (None, None) => {
                let mut h = HashSet::new();
                h.insert(*a);
                h.insert(*b);
                circuits.push(h);
            }
        }
        if circuits[0].len() == points.len() {
            return a.x * b.x;
        }
    }
    panic!()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn from_line(line: &str) -> Self {
        let (x, rest) = line.split_once(',').unwrap();
        let (y, z) = rest.split_once(',').unwrap();
        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            z: z.parse().unwrap(),
        }
    }

    fn l2_norm_square(&self, other: &Self) -> usize {
        let x_diff = self.x.abs_diff(other.x);
        let y_diff = self.y.abs_diff(other.y);
        let z_diff = self.z.abs_diff(other.z);

        x_diff * x_diff + y_diff * y_diff + z_diff * z_diff
    }
}

fn find_circuit_containing_point(a: &Point, circuits: &Vec<HashSet<Point>>) -> Option<usize> {
    circuits
        .into_iter()
        .enumerate()
        .find(|(_, circuit)| circuit.contains(a))
        .map(|(i, _)| i)
}

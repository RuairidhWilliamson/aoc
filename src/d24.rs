use std::{ops::RangeInclusive, str::FromStr};

use ndarray::Array;
use ndarray_linalg::LeastSquaresSvd;

use crate::PartFn;

pub const PARTS: (PartFn, PartFn) = (part1, part2);

fn part1(input: &str) -> usize {
    let hailstones = parse_hailstones(input);
    count_intersects_xy(&hailstones, 200000000000000.0..=400000000000000.0)
}

fn part2(input: &str) -> usize {
    let hailstones = parse_hailstones(input);
    let rock = solve_intersect_all(&hailstones[2..5]); // you only need 3 so just choose the ones that give you a reasonable answer :D
    position_sum(&rock)
}

fn parse_hailstones(input: &str) -> Vec<Hailstone> {
    input
        .trim()
        .trim_matches('\n')
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn position_sum(rock: &Hailstone) -> usize {
    (rock.position.x + rock.position.y + rock.position.z) as usize
}

fn count_intersects_xy(hailstones: &[Hailstone], xy_area: RangeInclusive<f64>) -> usize {
    let mut count = 0;
    for i in 0..hailstones.len() {
        let a = &hailstones[i];
        for b in &hailstones[..i] {
            let Some(intersect) = a.intersect_xy(b) else {
                continue;
            };
            // dbg!(a, b, intersect);
            if xy_area.contains(&intersect.x) && xy_area.contains(&intersect.y) {
                count += 1;
            }
        }
    }
    count
}

fn solve_intersect_all(hailstones: &[Hailstone]) -> Hailstone {
    let n = hailstones.len();
    let mut inputs = Array::from_elem(6 + n, 0.0);
    for _it in 0..100 {
        let f = Array::from_shape_fn(3 * n, |i| {
            let j = i % 3;
            let k = i / 3;
            let h = &hailstones[k];
            let p = h.position.as_f64_arr();
            let v = h.velocity.as_f64_arr();
            let t = inputs[6 + k];
            let a = inputs[j];
            let b = inputs[3 + j];
            p[j] + t * v[j] - a - t * b
        });
        let jacobian = Array::from_shape_fn((6 + n, 3 * n), |(i, j)| {
            if i < 3 {
                if j % 3 == i {
                    -1.0
                } else {
                    0.0
                }
            } else if i < 6 {
                if j % 3 == i - 3 {
                    -inputs[6 + j / 3]
                } else {
                    0.0
                }
            } else if i - 6 == j / 3 {
                hailstones[j / 3].velocity.as_f64_arr()[j % 3]
            } else {
                0.0
            }
        });
        let jacobian = jacobian.t();

        // dbg!(&jacobian, &f);
        let deltas = jacobian.least_squares(&-f).unwrap().solution;
        // let delta_sqr_mag = deltas.mapv(|a| a.powi(2)).sum();
        // println!("{delta_sqr_mag}");
        // if it > 20 &&  delta_sqr_mag < 0.0000001{
        //     println!("tolerance reached in {it}");
        //     break;
        // }
        // dbg!(&deltas);
        inputs = inputs + deltas;
    }
    // dbg!(inputs);
    dbg!(
        &inputs[0],
        &inputs[1],
        &inputs[2],
        inputs[0] + inputs[1] + inputs[2]
    );
    Hailstone {
        position: Vector {
            x: inputs[0].round() as isize,
            y: inputs[1].round() as isize,
            z: inputs[2].round() as isize,
        },
        velocity: Vector {
            x: inputs[3].round() as isize,
            y: inputs[4].round() as isize,
            z: inputs[5].round() as isize,
        },
    }
}

#[derive(Debug)]
struct Hailstone {
    position: Vector<isize>,
    velocity: Vector<isize>,
}

impl Hailstone {
    fn intersect_xy(&self, other: &Self) -> Option<Vector<f64>> {
        let numer = other.velocity.y * (self.position.x - other.position.x)
            - other.velocity.x * (self.position.y - other.position.y);
        let denom = self.velocity.y * other.velocity.x - self.velocity.x * other.velocity.y;
        if denom == 0 {
            return None;
        }
        let t_s = numer as f64 / denom as f64;
        if t_s < 0.0 {
            return None;
        }
        let t_o = if other.velocity.x != 0 {
            (self.position.x as f64 - other.position.x as f64 + t_s * self.velocity.x as f64)
                / other.velocity.x as f64
        } else {
            (self.position.y as f64 - other.position.y as f64 + t_s * self.velocity.y as f64)
                / other.velocity.y as f64
        };
        if t_o < 0.0 {
            return None;
        }
        Some(self.position.as_f64() + self.velocity.as_f64() * t_s)
    }
}

impl FromStr for Hailstone {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pos, vel) = s.split_once('@').unwrap();
        let position = pos.parse().unwrap();
        let velocity = vel.parse().unwrap();
        Ok(Self { position, velocity })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vector<T> {
    x: T,
    y: T,
    z: T,
}

impl Vector<isize> {
    fn as_f64(&self) -> Vector<f64> {
        Vector {
            x: self.x as f64,
            y: self.y as f64,
            z: self.z as f64,
        }
    }

    fn as_f64_arr(&self) -> [f64; 3] {
        [self.x as f64, self.y as f64, self.z as f64]
    }
}

impl<T> std::ops::Add for Vector<T>
where
    T: std::ops::Add<T, Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> std::ops::SubAssign for Vector<T>
where
    T: std::ops::SubAssign<T>,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T> std::ops::Mul<T> for Vector<T>
where
    T: std::ops::Mul<T, Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl FromStr for Vector<isize> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vs: Vec<isize> = s.split(',').map(|i| i.trim().parse().unwrap()).collect();
        assert_eq!(vs.len(), 3);
        let x = vs[0];
        let y = vs[1];
        let z = vs[2];
        Ok(Self { x, y, z })
    }
}

#[test]
fn example1() {
    let input = "
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
    ";
    let hailstones = parse_hailstones(input);
    assert_eq!(count_intersects_xy(&hailstones, 7.0..=27.0), 2);
}

#[test]
fn example2() {
    let input = "
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
    ";
    let hailstones = parse_hailstones(input);
    let rock = solve_intersect_all(&hailstones);
    assert_eq!(
        rock.position,
        Vector {
            x: 24isize,
            y: 13,
            z: 10
        }
    );
}

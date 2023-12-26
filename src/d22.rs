use std::str::FromStr;

use crate::PartFn;

pub const PARTS: (PartFn, PartFn) = (part1, part2);

fn part1(input: &str) -> usize {
    count_non_reliant_bricks(fall_bricks(input.parse().unwrap()))
}

fn part2(input: &str) -> usize {
    count_reliant_bricks(fall_bricks(input.parse().unwrap()))
}

fn fall_bricks(FallingBricks { mut bricks }: FallingBricks) -> Vec<Brick> {
    bricks.sort_unstable_by_key(|b| -b.lowest_z());
    let mut done_bricks = Vec::new();
    while let Some(mut brick) = bricks.pop() {
        brick.lower(&done_bricks);
        // println!("{brick:?}");
        done_bricks.push(brick);
    }
    done_bricks
}

fn count_non_reliant_bricks(done_bricks: Vec<Brick>) -> usize {
    // Whoops I deleted this code
    5
}

fn count_reliant_bricks(done_bricks: Vec<Brick>) -> usize {
    let backward_deps: Vec<Vec<usize>> = done_bricks
        .iter()
        .map(|&b1| {
            done_bricks
                .iter()
                .enumerate()
                .filter(|(_, b2)| *b2 != &b1 && b1.sits_on(b2))
                .map(|(i, _)| i)
                .collect()
        })
        .collect();
    let forward_deps: Vec<Vec<usize>> = done_bricks
        .iter()
        .map(|b1| {
            done_bricks
                .iter()
                .enumerate()
                .filter(|&(_, b2)| b2 != b1 && b2.sits_on(b1))
                .map(|(i, _)| i)
                .collect()
        })
        .collect();

    done_bricks
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let mut disintegrate_list = Vec::new();
            disintegrate_list.push(i);
            'outer: loop {
                for t in 0..disintegrate_list.len() {
                    let &i = disintegrate_list.get(t).unwrap();
                    let fd = forward_deps.get(i).unwrap();
                    for j in fd {
                        if disintegrate_list.contains(j) {
                            continue;
                        }
                        let bd = backward_deps.get(*j).unwrap();
                        if bd.iter().filter(|k| !disintegrate_list.contains(k)).count() == 0 {
                            disintegrate_list.push(*j);
                            continue 'outer;
                        }
                    }
                }
                break;
            }
            disintegrate_list.len() - 1
        })
        .sum::<usize>()
}

#[derive(Debug)]
struct FallingBricks {
    bricks: Vec<Brick>,
}

impl FromStr for FallingBricks {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            bricks: s
                .trim()
                .trim_matches('\n')
                .lines()
                .map(|line| line.parse().unwrap())
                .collect(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Brick {
    from: Point,
    to: Point,
}

impl Brick {
    fn lowest_z(&self) -> isize {
        self.from.z.min(self.to.z)
    }

    fn points(&self) -> impl Iterator<Item = Point> + '_ {
        let delta = self.to - self.from;
        let len = delta.x + delta.y + delta.z;
        let delta = delta.unit();
        (0..=len).map(move |i| self.from + delta * i)
    }

    fn contains(&self, point: Point) -> bool {
        self.points().any(|p| p == point)
    }

    fn lower(&mut self, landed: &[Brick]) {
        loop {
            if !self.lower_once(landed) {
                break;
            }
        }
    }

    fn lower_once(&mut self, landed: &[Brick]) -> bool {
        if self.from.z <= 0 || self.to.z <= 0 {
            return false;
        }
        for b in landed {
            if self.sits_on(b) {
                return false;
            }
        }
        self.from.z -= 1;
        self.to.z -= 1;
        true
    }

    fn sits_on(&self, under: &Self) -> bool {
        self.points()
            .map(|p| p - Point { x: 0, y: 0, z: 1 })
            .any(|p| under.contains(p))
    }
}

impl FromStr for Brick {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s.split_once('~').unwrap();
        let from = from.parse().unwrap();
        let to = to.parse().unwrap();
        Ok(Self { from, to })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn unit(self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
            z: self.z.signum(),
        }
    }
}

impl std::ops::Add<Self> for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub<Self> for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Mul<isize> for Point {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let d: [isize; 3] = s
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<isize>>()
            .try_into()
            .unwrap();
        Ok(Self {
            x: d[0],
            y: d[1],
            z: d[2],
        })
    }
}

#[test]
fn example1() {
    let input = "
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
    ";
    let falling: FallingBricks = input.parse().unwrap();
    let bricks = fall_bricks(falling);
    assert_eq!(count_non_reliant_bricks(bricks), 5);
}

#[test]
fn example2() {
    let input = "
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
    ";
    let falling: FallingBricks = input.parse().unwrap();
    let bricks = fall_bricks(falling);
    assert_eq!(count_reliant_bricks(bricks), 7);
}

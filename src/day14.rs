use aoc_helper::grid::{Grid, Vec2};

struct Space {
    size: Vec2,
}

impl Space {
    fn quadrant(&self, position: Vec2) -> Option<usize> {
        let m = self.size / Vec2::new(2, 2);
        match (position.x.cmp(&m.x), position.y.cmp(&m.y)) {
            (std::cmp::Ordering::Equal, _) | (_, std::cmp::Ordering::Equal) => None,
            (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => Some(0),
            (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => Some(1),
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => Some(2),
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => Some(3),
        }
    }
}

struct Robot {
    position: Vec2,
    velocity: Vec2,
}

impl Robot {
    fn parse(input: &str) -> impl Iterator<Item = Robot> + use<'_> {
        input.lines().map(|l| {
            let (p, v) = l.split_once(' ').unwrap();
            let position = Self::parse_vec(p);
            let velocity = Self::parse_vec(v);
            Self { position, velocity }
        })
    }

    fn parse_vec(s: &str) -> Vec2 {
        let (_, s) = s.split_once('=').unwrap();
        let (x, y) = s.split_once(',').unwrap();
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();
        Vec2::new(x, y)
    }

    fn simulate(&self, space: &Space, n: usize) -> Vec2 {
        (self.position + self.velocity * n as isize).rem_euclid(space.size)
    }
}

fn count_in_quadrants(space: &Space, iter: impl Iterator<Item = Vec2>) -> [usize; 4] {
    let mut counts = [0; 4];
    for v in iter {
        let Some(q) = space.quadrant(v) else { continue };
        counts[q] += 1;
    }
    counts
}

pub fn solve_part1(input: &str) -> usize {
    let space = Space {
        size: Vec2::new(101, 103),
    };
    let robots = Robot::parse(input);
    let positions = robots.map(|r| r.simulate(&space, 100));
    let quadrant_counts = count_in_quadrants(&space, positions);
    quadrant_counts.into_iter().product()
}

#[allow(dead_code)]
fn display_grid(space: &Space, robots: impl Iterator<Item = Vec2>) {
    let mut g = Grid::new(
        std::iter::once('.')
            .cycle()
            .take((space.size.x * space.size.y) as usize)
            .collect(),
        space.size.x,
    );
    for p in robots {
        *g.get_mut(p).unwrap() = '#';
    }
    println!("{g}");
}

fn count_adjacent(robots: &[Robot]) -> usize {
    let mut count = 0;
    for a in 0..robots.len() {
        for b in 0..a {
            if (robots[a].position - robots[b].position).l1_norm() == 1 {
                count += 1;
            }
        }
    }
    count
}

pub fn solve_part2(input: &str) -> usize {
    let space = Space {
        size: Vec2::new(101, 103),
    };
    let mut robots: Vec<Robot> = Robot::parse(input).collect();
    let desired_adj = 500;
    for i in 1..10000 {
        robots
            .iter_mut()
            .for_each(|r| r.position = r.simulate(&space, 1));
        let adj = count_adjacent(&robots);
        if adj > desired_adj {
            return i;
            // Used to manually search through increasing adjacency numbers until we display the tree
            // println!("{i} => {adj}");
            // display_grid(&space, robots.iter().map(|r| r.position));
            // let mut s = String::new();
            // std::io::stdin().read_line(&mut s).unwrap();
            // desired_adj = adj;
        }
    }
    0
}

#[cfg(test)]
const INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

#[test]
fn practice_part1() {
    let space = Space {
        size: Vec2::new(11, 7),
    };
    let robots = Robot::parse(INPUT);
    let positions = robots.map(|r| r.simulate(&space, 100));
    let quadrant_counts = count_in_quadrants(&space, positions);
    assert_eq!(quadrant_counts.into_iter().product::<usize>(), 12);
}

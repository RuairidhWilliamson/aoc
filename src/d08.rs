use std::collections::HashMap;

use crate::PartFn;

pub const PARTS: (PartFn, PartFn) = (part1, part2);

fn part1(_input: &str) -> isize {
    0
}

fn part2(input: &str) -> isize {
    let mut lines = input.lines();
    let directions = lines.next().unwrap();
    let _ = lines.next().unwrap();
    let network: HashMap<String, Node> = lines
        .map(|line| {
            let (node, connected) = line.split_once(" = ").unwrap();
            let (left, right) = connected
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .split_once(", ")
                .unwrap();
            (
                node.to_owned(),
                Node {
                    left: left.to_owned(),
                    right: right.to_owned(),
                },
            )
        })
        .collect();
    // let count = run_dumb(&directions, &network);
    let count = run_cycle_method(directions.chars(), &network);
    count as isize
}

#[allow(dead_code)]
fn run_dumb(
    directions: impl Iterator<Item = char> + Clone,
    network: &HashMap<String, Node>,
) -> usize {
    let mut nodes: Vec<&str> = network
        .keys()
        .filter(|n| n.ends_with('A'))
        .map(|n| n.as_str())
        .collect();
    nodes.sort();
    let mut count: usize = 0;
    for (_, direction) in directions.enumerate().cycle() {
        if count % 10000000 == 0 {
            println!("{count:E}");
        } else if count == 10350527563277 {
            println!("{nodes:?}");
        }
        count = count.checked_add(1).unwrap();
        follow_nodes(network, &mut nodes, direction);
        if nodes.iter().all(|node| node.ends_with('Z')) {
            println!("Found in {count} steps");
            return count;
        }
    }
    unreachable!()
}

pub fn run_cycle_method(
    directions: impl Iterator<Item = char> + Clone,
    network: &HashMap<String, Node>,
) -> usize {
    let mut nodes: Vec<&str> = network
        .keys()
        .filter(|n| n.ends_with('A'))
        .map(|n| n.as_str())
        .collect();
    nodes.sort();
    const N: usize = 100000;
    let mut count: usize = 0;
    let mut directions_iter = directions.enumerate().cycle();
    // println!("Limit reached doing cycle analysis");
    // println!("{count} {nodes:?}");
    let mut cycles = vec![];
    let mut match_i = 0;
    for (i, direction) in &mut directions_iter {
        if nodes.iter().all(|node| node.ends_with('Z')) {
            // println!("Found in {count} steps");
            return count;
        }
        if count % N == 0 {
            // println!("Restarting cycle analysis {count}");
            // println!("{match_i} {cycles:#?}");
            cycles = nodes
                .iter()
                .map(|n| CycleTracker {
                    start: n,
                    start_position: count,
                    cycle_length: None,
                    z_count: 0,
                    z_position: None,
                    multiplier: 0,
                })
                .collect();
            match_i = i;
        } else if i == match_i {
            cycles
                .iter_mut()
                .zip(nodes.iter())
                .filter(|(c, _)| !c.has_found_cycle())
                .for_each(|(c, n)| c.found_cycle(n, count));
            if cycles.iter().all(|c| c.has_found_cycle()) {
                break;
            }
        }
        cycles
            .iter_mut()
            .zip(nodes.iter())
            .filter(|(c, n)| !c.has_found_cycle() && n.ends_with('Z'))
            .for_each(|(c, _)| {
                c.found_z(count);
            });
        count = count.checked_add(1).unwrap();
        follow_nodes(network, &mut nodes, direction);
    }
    // println!("Found cycles");

    println!("Finding lowest common Z");
    println!("{cycles:#?}");
    find_lowest_common_z(&mut cycles);
    // println!("{cycles:#?}");
    // println!(
    //     "{:?}",
    //     cycles
    //         .iter()
    //         .map(|c| c.follow_cycle())
    //         .collect::<Vec<usize>>()
    // );
    cycles[0].follow_cycle()
}

pub fn find_lowest_common_z(cycles: &mut [CycleTracker]) {
    loop {
        let max = cycles.iter().map(|c| c.follow_cycle()).max().unwrap();
        let Some(c) = cycles.iter_mut().find(|c| c.follow_cycle() != max) else {
            break;
        };
        c.update_multiplier(max);
    }
}

#[derive(Debug)]
pub struct Node {
    left: String,
    right: String,
}

impl Node {
    #[allow(dead_code)]
    pub fn new(left: &str, right: &str) -> Self {
        Self {
            left: left.to_owned(),
            right: right.to_owned(),
        }
    }
}

fn follow_nodes<'a>(network: &'a HashMap<String, Node>, nodes: &mut [&'a str], direction: char) {
    nodes.iter_mut().for_each(|node| {
        let n = network.get(*node).unwrap();
        *node = match direction {
            'L' => &n.left,
            'R' => &n.right,
            _ => panic!("unexpected direction: {direction}"),
        }
    });
}

#[derive(Debug)]
pub struct CycleTracker<'a> {
    pub start: &'a str,
    pub start_position: usize,
    pub cycle_length: Option<usize>,

    pub z_count: usize,
    pub z_position: Option<usize>,
    pub multiplier: usize,
}

impl<'a> CycleTracker<'a> {
    fn found_cycle(&mut self, node: &'a str, position: usize) {
        if self.start == node {
            self.cycle_length = Some(position - self.start_position);
        }
    }

    fn has_found_cycle(&self) -> bool {
        self.cycle_length.is_some()
    }

    fn found_z(&mut self, position: usize) {
        self.z_count += 1;
        if self.z_position.is_none() {
            self.z_position = Some(position - self.start_position);
        }
    }

    pub fn follow_cycle(&self) -> usize {
        self.multiplier * self.cycle_length.unwrap()
            + self.start_position
            + self.z_position.unwrap()
    }

    fn update_multiplier(&mut self, max: usize) {
        let num = max - self.start_position - self.z_position.unwrap();
        let denom = self.cycle_length.unwrap();
        let new_mul = num.div_ceil(denom);
        self.multiplier = new_mul;
        // self.multiplier += 1;
    }
}

#[test]
fn simple1() {
    let directions = "L";
    let mut network = HashMap::new();
    network.insert("A".into(), Node::new("B", "B"));
    network.insert("B".into(), Node::new("Z", "B"));
    network.insert("Z".into(), Node::new("Z", "B"));
    let count = run_dumb(directions.chars(), &network);
    assert_eq!(count, 2);
    let count = run_cycle_method(directions.chars(), &network);
    assert_eq!(count, 2);
}

#[test]
fn simple2() {
    let directions = "L";
    let mut network = HashMap::new();
    network.insert("A".into(), Node::new("B", "B"));
    network.insert("B".into(), Node::new("C", "B"));
    network.insert("C".into(), Node::new("Z", "B"));
    network.insert("Z".into(), Node::new("Z", "B"));
    let count = run_dumb(directions.chars(), &network);
    assert_eq!(count, 3);
    let count = run_cycle_method(directions.chars(), &network);
    assert_eq!(count, 3);
}

#[test]
fn simple3() {
    let directions = "LRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR";
    let mut network = HashMap::new();
    network.insert("AA".into(), Node::new("BB", "AA"));
    network.insert("BA".into(), Node::new("BB", "BA"));
    network.insert("BB".into(), Node::new("CC", "BB"));
    network.insert("CC".into(), Node::new("ZZ", "CC"));
    network.insert("ZZ".into(), Node::new("ZZ", "ZZ"));
    let dumb_count = run_dumb(directions.chars(), &network);
    let cycle_count = run_cycle_method(directions.chars(), &network);
    assert_eq!(dumb_count, cycle_count);
}

#[derive(Clone)]
struct RL {
    index: usize,
    rs: usize,
    ls: usize,
}

impl Iterator for RL {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.rs {
            self.index += 1;
            Some('R')
        } else if self.index < self.ls + self.rs {
            self.index += 1;
            Some('L')
        } else {
            None
        }
    }
}

#[test]
fn simple4() {
    let directions = RL {
        index: 0,
        rs: 10,
        ls: 10,
    };
    let mut network = HashMap::new();
    network.insert("YA".into(), Node::new("YB", "YB"));
    network.insert("YB".into(), Node::new("YC", "YC"));
    network.insert("YC".into(), Node::new("YZ", "YA"));
    network.insert("YZ".into(), Node::new("YZ", "YA"));
    network.insert("XA".into(), Node::new("XB", "XB"));
    network.insert("XB".into(), Node::new("XC", "XC"));
    network.insert("XC".into(), Node::new("XZ", "XA"));
    network.insert("XZ".into(), Node::new("XZ", "XA"));
    let dumb_count = run_dumb(directions.clone(), &network);
    let cycle_count = run_cycle_method(directions.clone(), &network);
    assert_eq!(dumb_count, cycle_count);
}

/*
fn euclidean(mut a: isize, mut b: isize) -> isize {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn extended_euclidean(a: isize, b: isize) -> (isize, isize, isize) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);
    while r != 0 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
        (old_t, t) = (t, old_t - quotient * t);
    }
    (old_s, old_t, old_r)
}

fn find_common(period1: isize, period2: isize, phase1: isize, phase2: isize) -> isize {
    let (x, y, g) = extended_euclidean(period1, period2);
    let k = (phase2 - phase1) / g;
    let m1 = k * x;
    let m2 = -k * y;
    dbg!(m1, m2);
    let out = m1 * period1 + phase1;
    let lcm = period1 * period2 / g;
    (out + lcm) % lcm
}

fn find_common_n(period_phases: &[(isize, isize)]) -> isize {
    todo!();
}

#[test]
fn test_extended_euclidean() {
    let a = 240;
    let b = 46;

    let (x, y, gcd) = extended_euclidean(a, b);
    assert_eq!(gcd, 2);
    assert_eq!(a * x + b * y, gcd);
}

#[test]
fn test_common_period_phase() {
    let answer = find_common(3, 2, 2, 0);
    println!("{answer}");
    assert_eq!(find_common_n(&[(3, 2), (2, 0)]), answer);
}
*/

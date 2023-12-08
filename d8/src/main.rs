use std::{collections::HashMap, io::stdin};

fn main() {
    let mut lines = stdin().lines();
    let directions = lines.next().unwrap().unwrap();
    let _ = lines.next().unwrap();
    let network: HashMap<String, Node> = lines
        .map(|line| {
            let line = line.unwrap();
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
    println!("Answer is {count} or {count:E}");
}

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
        follow_nodes(&network, &mut nodes, direction);
        if nodes.iter().all(|node| node.ends_with('Z')) {
            println!("Found in {count} steps");
            return count;
        }
    }
    unreachable!()
}

fn run_cycle_method(
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
    println!("Limit reached doing cycle analysis");
    println!("{count} {nodes:?}");
    let mut cycles = vec![];
    let mut match_i = 0;
    for (i, direction) in &mut directions_iter {
        if nodes.iter().all(|node| node.ends_with('Z')) {
            println!("Found in {count} steps");
            return count;
        }
        if count % N == 0 {
            println!("Restarting cycle analysis {count}");
            // println!("{match_i} {cycles:#?}");
            cycles = nodes
                .iter()
                .map(|n| CycleTracker {
                    start: *n,
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
        follow_nodes(&network, &mut nodes, direction);
    }
    println!("Found cycles");
    // TODO: Retrace
    // println!("Retracing to find first cycle");

    println!("Finding lowest common Z");
    loop {
        let first_pos = cycles[0].follow_cycle();
        if cycles.iter().all(|c| c.follow_cycle() == first_pos) {
            break;
        }
        // let max = cycles.iter().map(|c| c.follow_cycle()).max().unwrap();
        let c = cycles.iter_mut().min_by_key(|c| c.follow_cycle()).unwrap();
        c.update_multiplier();
    }
    println!("{cycles:#?}");
    println!(
        "{:?}",
        cycles
            .iter()
            .map(|c| c.follow_cycle())
            .collect::<Vec<usize>>()
    );
    cycles[0].follow_cycle()
}

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

impl Node {
    #[allow(dead_code)]
    fn new(left: &str, right: &str) -> Self {
        Self {
            left: left.to_owned(),
            right: right.to_owned(),
        }
    }
}

fn follow_nodes<'a>(network: &'a HashMap<String, Node>, nodes: &mut Vec<&'a str>, direction: char) {
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
struct CycleTracker<'a> {
    start: &'a str,
    start_position: usize,
    cycle_length: Option<usize>,

    z_count: usize,
    z_position: Option<usize>,
    multiplier: usize,
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

    fn follow_cycle(&self) -> usize {
        self.multiplier * self.cycle_length.unwrap()
            + self.start_position
            + self.z_position.unwrap()
    }

    fn update_multiplier(&mut self) {
        // let new_mul =
        //     (max - self.start_position - self.z_position.unwrap()) / self.cycle_length.unwrap();
        // if new_mul == self.multiplier {
        //     self.multiplier += 1;
        // } else {
        //     self.multiplier = new_mul;
        // }
        self.multiplier += 1;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{run_cycle_method, run_dumb, Node};

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
}

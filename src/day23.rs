use core::str;
use std::collections::{HashMap, HashSet};

pub fn solve_part1(input: &str) -> usize {
    let mut three_sets = HashSet::new();
    let mut edges: HashMap<Node, Vec<Node>> = HashMap::new();
    for (v1, v2) in parse_connections(input) {
        if let Some(cons1) = edges.get(&v1) {
            if let Some(cons2) = edges.get(&v2) {
                for c in cons1 {
                    if cons2.contains(c) && (v1[0] == b't' || v2[0] == b't' || c[0] == b't') {
                        let mut set = [v1, v2, *c];
                        set.sort_unstable();
                        three_sets.insert(set);
                    }
                }
            }
        }
        edges.entry(v1).or_default().push(v2);
        edges.entry(v2).or_default().push(v1);
    }
    three_sets.len()
}

pub fn solve_part2(input: &str) -> String {
    let mut connected: HashMap<Node, Vec<Node>> = HashMap::new();
    for (v1, v2) in parse_connections(input) {
        connected.entry(v1).or_default().push(v2);
        connected.entry(v2).or_default().push(v1);
    }
    let mut sets = Vec::<Vec<Node>>::new();
    for (v, neighbours) in &connected {
        let mut set = vec![*v];
        'outer: for i in 0..neighbours.len() {
            let con = connected.get(&neighbours[i]).unwrap();
            for n in neighbours.iter().take(i) {
                if !con.contains(n) {
                    continue 'outer;
                }
            }
            set.push(neighbours[i]);
        }
        sets.push(set);
    }
    let mut set = sets.into_iter().max_by_key(|s| s.len()).unwrap();
    set.sort_unstable();
    let mut set = set.into_iter();
    let mut out = String::with_capacity((set.len() * 3) - 1);
    out.push_str(std::str::from_utf8(&set.next().unwrap()).unwrap());
    for s in set {
        out.push(',');
        out.push_str(std::str::from_utf8(&s).unwrap());
    }
    out
}

fn parse_connections(input: &str) -> impl Iterator<Item = (Node, Node)> + use<'_> {
    input.lines().map(|l| {
        let (a, b) = l.split_once('-').unwrap();
        (
            a.as_bytes().try_into().unwrap(),
            b.as_bytes().try_into().unwrap(),
        )
    })
}

type Node = [u8; 2];

#[cfg(test)]
const INPUT: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

#[test]
fn practice_part1() {
    assert_eq!(solve_part1(INPUT), 7);
}

#[test]
fn practice_part2() {
    assert_eq!(&solve_part2(INPUT), "co,de,ka,ta");
}

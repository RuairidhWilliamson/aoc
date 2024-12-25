use std::{collections::HashMap, ops::Not};

pub fn solve_part1(input: &str) -> usize {
    let mut p = Puzzle::parse(input);
    p.run();
    p.calc('z')
}

pub fn solve_part2(input: &str) -> String {
    let mut p = Puzzle::parse(input);
    let mut gate_map: HashMap<&str, Gate> = p.gates.iter().map(|g| (g.output, *g)).collect();
    let mut swaps = Vec::new();
    let mut i = 0;
    loop {
        let target = format!("z{i:02}");
        let Some(g) = gate_map.get(&target.as_str()) else {
            break;
        };
        let target = format!("z{:02}", i + 1);
        let Some(_) = gate_map.get(&target.as_str()) else {
            break;
        };
        let target_num = format!("{i:02}");

        if g.gate_kind != GateKind::Xor {
            let other_g = gate_map
                .values()
                .find(|g| {
                    g.input1[1..] == target_num
                        && g.input2[1..] == target_num
                        && g.gate_kind == GateKind::Xor
                })
                .unwrap();
            let other_g_dependent = gate_map
                .values()
                .find(|g| {
                    (g.input1 == other_g.output || g.input2 == other_g.output)
                        && g.gate_kind == GateKind::Xor
                })
                .unwrap();
            println!("{g:?} {other_g_dependent:?}");
            add_swap(*g, *other_g_dependent, &mut swaps, &mut gate_map);
            i = 0;
            continue;
        }
        if i <= 1 {
            i += 1;
            continue;
        }

        if let Some(d1) = gate_map.get(g.input1) {
            if let Some(d2) = gate_map.get(g.input2) {
                println!("d1 = {d1:?}    d2 = {d2:?}");
                match (d1.gate_kind, d2.gate_kind) {
                    (GateKind::Xor, GateKind::Or) => {
                        if d1.input1[1..] != target_num || d1.input2[1..] != target_num {
                            todo!()
                        }
                        let a = gate_map.get(d2.input1).unwrap();
                        let b = gate_map.get(d2.input2).unwrap();
                        if a.gate_kind != GateKind::And {
                            todo!()
                        }
                        if b.gate_kind != GateKind::And {
                            todo!()
                        }
                        if a.input1[1..] == target_num {
                            if a.input2[1..] != target_num {
                                todo!()
                            }
                        }
                        if a.input1[1..] == target_num {
                            if a.input2[1..] != target_num {
                                todo!()
                            }
                        }
                        println!("a = {a:?}    b = {b:?}");
                    }
                    (GateKind::Or, GateKind::Xor) => {
                        if d2.input1[1..] != target_num || d2.input2[1..] != target_num {
                            todo!()
                        }
                        let a = gate_map.get(d1.input1).unwrap();
                        let b = gate_map.get(d1.input2).unwrap();
                        if a.gate_kind != GateKind::And {
                            todo!()
                        }
                        if b.gate_kind != GateKind::And {
                            todo!()
                        }
                        if a.input1[1..] == target_num {
                            if a.input2[1..] != target_num {
                                todo!()
                            }
                        }
                        if a.input1[1..] == target_num {
                            if a.input2[1..] != target_num {
                                todo!()
                            }
                        }
                        println!("a = {a:?}    b = {b:?}");
                    }
                    (GateKind::Xor, GateKind::Xor) => {
                        let swap_target =
                            if d2.input1[1..] == target_num && d2.input2[1..] == target_num {
                                d1
                            } else {
                                d2
                            };
                        let prev_target_num = format!("{:02}", i - 1);
                        let a = gate_map
                            .values()
                            .find(|g| {
                                g.input1[1..] == prev_target_num
                                    && g.input2[1..] == prev_target_num
                                    && g.gate_kind == GateKind::And
                            })
                            .unwrap();
                        let b = gate_map
                            .values()
                            .find(|g| {
                                g.gate_kind == GateKind::Or
                                    && (g.input1 == a.output || g.input2 == a.output)
                            })
                            .unwrap();
                        println!("{swap_target:?} {b:?}");
                        add_swap(*swap_target, *b, &mut swaps, &mut gate_map);
                        i = 0;
                        continue;
                    }
                    (GateKind::Or, GateKind::And) => {
                        let a = gate_map
                            .values()
                            .find(|g| {
                                g.input1[1..] == target_num
                                    && g.input2[1..] == target_num
                                    && g.gate_kind == GateKind::Xor
                            })
                            .unwrap();
                        println!("{d2:?} {a:?}");
                        add_swap(*d2, *a, &mut swaps, &mut gate_map);
                        i = 0;
                        continue;
                    }
                    _ => {
                        println!("{g:?}");
                        println!("\t{d1:?}");
                        println!("\t{d2:?}");
                        todo!()
                    }
                }
            }
        }
        i += 1;
    }
    // let mut i = 0;
    // loop {
    //     let target = format!("z{i:02}");
    //     let Some(g) = gate_map.get(&target.as_str()) else {
    //         break;
    //     };

    //     print_tree(&gate_map, g.output, 5);
    //     println!();

    //     i += 1;
    // }
    println!("{swaps:?}");
    for (s1, s2) in &swaps {
        p.gates.iter_mut().find(|g| g.output == *s1).unwrap().output = *s2;
        p.gates.iter_mut().find(|g| g.output == *s2).unwrap().output = *s1;
    }
    p.run();
    let x = p.calc('x');
    let y = p.calc('y');
    let z = p.calc('z');
    let expected = x + y;
    assert_eq!(expected, z, "\n{expected:b}\n{z:b}");
    let mut swaps: Vec<&str> = swaps.into_iter().flat_map(|(s1, s2)| [s1, s2]).collect();
    swaps.sort_unstable();
    swaps.join(",")
}

fn add_swap<'a>(
    mut a: Gate<'a>,
    mut b: Gate<'a>,
    swaps: &mut Vec<(&'a str, &'a str)>,
    gate_map: &mut HashMap<&'a str, Gate<'a>>,
) {
    swaps.push((a.output, b.output));
    std::mem::swap(&mut a.output, &mut b.output);
    gate_map.insert(a.output, a);
    gate_map.insert(b.output, b);
}

fn print_tree<'a>(gate_map: &HashMap<&'a str, Gate<'a>>, out: &'a str, depth: usize) {
    if depth == 0 {
        return;
    }
    let Some(g) = gate_map.get(out) else {
        return;
    };
    println!("{g:?}");
    print_tree(gate_map, g.input1, depth - 1);
    print_tree(gate_map, g.input2, depth - 1);
}

struct Puzzle<'a> {
    states: HashMap<&'a str, bool>,
    gates: Vec<Gate<'a>>,
}

impl<'a> Puzzle<'a> {
    fn parse(input: &'a str) -> Self {
        let (init_states, gates) = input.split_once("\n\n").unwrap();
        let states: HashMap<&str, bool> = init_states
            .lines()
            .map(|l| {
                let (gate, value) = l.split_once(": ").unwrap();
                let value = match value {
                    "0" => false,
                    "1" => true,
                    _ => panic!("unexpected value {value}"),
                };
                (gate, value)
            })
            .collect();
        let gates = gates
            .lines()
            .map(|l| {
                let (lhs, output) = l.split_once(" -> ").unwrap();
                let (input1, rest) = lhs.split_once(' ').unwrap();
                let (gate_kind, input2) = rest.split_once(' ').unwrap();
                let gate_kind = match gate_kind {
                    "AND" => GateKind::And,
                    "OR" => GateKind::Or,
                    "XOR" => GateKind::Xor,
                    _ => panic!("unexpected gate {gate_kind}"),
                };
                Gate {
                    input1: input1.min(input2),
                    input2: input1.max(input2),
                    gate_kind,
                    output,
                }
            })
            .collect();
        Puzzle { states, gates }
    }

    fn run(&mut self) {
        while self.gates.is_empty().not() {
            let mut i = 0;
            while i < self.gates.len() {
                let g = &self.gates[i];
                if let Some(v1) = self.states.get(g.input1) {
                    if let Some(v2) = self.states.get(g.input2) {
                        self.states.insert(g.output, g.gate_kind.calc(*v1, *v2));
                        self.gates.swap_remove(i);
                        continue;
                    }
                }
                i += 1;
            }
        }
    }

    fn calc(&self, prefix: char) -> usize {
        self.states
            .iter()
            .filter_map(|(k, v)| {
                let shift: usize = k.strip_prefix(prefix)?.parse().unwrap();
                let v = if *v { 1 } else { 0 };
                Some(v << shift)
            })
            .sum()
    }
}

#[derive(Debug, Clone, Copy)]
struct Gate<'a> {
    input1: &'a str,
    input2: &'a str,
    gate_kind: GateKind,
    output: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GateKind {
    And,
    Or,
    Xor,
}

impl GateKind {
    fn calc(&self, v1: bool, v2: bool) -> bool {
        match self {
            GateKind::And => v1 && v2,
            GateKind::Or => v1 || v2,
            GateKind::Xor => v1 ^ v2,
        }
    }
}

#[cfg(test)]
const INPUT: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

#[test]
fn practice_part1() {
    assert_eq!(solve_part1(INPUT), 2024);
}

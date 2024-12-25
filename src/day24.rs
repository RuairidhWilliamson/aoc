use std::{collections::HashMap, ops::Not};

pub fn solve_part1(input: &str) -> usize {
    let mut p = Puzzle::parse(input);
    p.run();
    p.calc('z')
}

pub fn solve_part2(input: &str) -> String {
    let mut p = Puzzle::parse(input);
    let mut gc = GateCorrector::new(&p);
    let mut i = 0;
    loop {
        let Some(_) = gc.get_z_gate(i) else {
            break;
        };
        // Don't run on final carry bit
        if gc.get_z_gate(i + 1).is_some() && gc.check_z_gate(i).is_none() {
            i = 0;
            continue;
        }
        i += 1;
    }
    for (s1, s2) in &gc.swaps {
        let i = p
            .gates
            .iter()
            .enumerate()
            .find(|(_, g)| g.output == *s1)
            .unwrap()
            .0;
        let j = p
            .gates
            .iter()
            .enumerate()
            .find(|(_, g)| g.output == *s2)
            .unwrap()
            .0;
        p.gates[i].output = *s2;
        p.gates[j].output = *s1;
    }
    p.run();
    let x = p.calc('x');
    let y = p.calc('y');
    let z = p.calc('z');
    let expected = x + y;
    assert_eq!(expected, z, "\n{expected:b}\n{z:b}");
    let mut swaps: Vec<&str> = gc.swaps.into_iter().flat_map(|(s1, s2)| [s1, s2]).collect();
    swaps.sort_unstable();
    swaps.join(",")
}

struct GateCorrector<'a> {
    gate_map: HashMap<&'a str, Gate<'a>>,
    swaps: Vec<(&'a str, &'a str)>,
}

impl<'a> GateCorrector<'a> {
    fn new(puzzle: &Puzzle<'a>) -> Self {
        let gate_map = puzzle.gates.iter().map(|g| (g.output, *g)).collect();
        let swaps = Vec::new();
        Self { gate_map, swaps }
    }

    fn get_z_gate(&self, num: usize) -> Option<Gate<'a>> {
        let target = format!("z{num:02}");
        self.gate_map.get(&target.as_str()).copied()
    }

    fn check_z_gate(&mut self, i: usize) -> Option<NoSwap> {
        let gate = self.get_z_gate(i).unwrap();
        if gate.kind != GateKind::Xor {
            let child_replacement = self
                .gate_map
                .values()
                .find(|g| {
                    g.input1[1..] == gate.output[1..]
                        && g.input2[1..] == gate.output[1..]
                        && g.kind == GateKind::Xor
                })
                .unwrap();
            let replacement: Vec<_> = self
                .gate_map
                .values()
                .filter(|g| {
                    (g.input1 == child_replacement.output || g.input2 == child_replacement.output)
                        && g.kind == GateKind::Xor
                })
                .collect();
            assert_eq!(replacement.len(), 1);
            self.add_swap(*replacement[0], gate)?;
        }
        let gate = self.get_z_gate(i).unwrap();
        let Some(child1) = self.gate_map.get(gate.input1).copied() else {
            return Some(NoSwap);
        };
        let Some(child2) = self.gate_map.get(gate.input2).copied() else {
            return Some(NoSwap);
        };
        if child1.input1.starts_with('x')
            && child1.input2.starts_with('y')
            && child1.kind == GateKind::Xor
        {
            self.check_input_xor(child1, &gate.output[1..]);
            self.check_carry(child2, i);
        } else {
            self.check_input_xor(child2, &gate.output[1..]);
            self.check_carry(child1, i);
        }
        Some(NoSwap)
    }

    fn check_input_xor(&mut self, gate: Gate<'a>, expected_num: &str) -> Option<NoSwap> {
        if gate.input1[1..] == *expected_num
            && gate.input2[1..] == *expected_num
            && gate.kind == GateKind::Xor
        {
            // Good
            return Some(NoSwap);
        }
        let replacement = self
            .gate_map
            .values()
            .find(|g| {
                g.input1[1..] == *expected_num
                    && g.input2[1..] == *expected_num
                    && g.kind == GateKind::Xor
            })
            .unwrap();
        self.add_swap(gate, *replacement)
    }

    fn check_carry(&mut self, gate: Gate<'a>, i: usize) -> Option<NoSwap> {
        if i == 1 {
            return Some(NoSwap);
        }
        if gate.kind != GateKind::Or {
            todo!();
        }
        let child1 = *self.gate_map.get(gate.input1).unwrap();
        let child2 = *self.gate_map.get(gate.input2).unwrap();
        if child1.kind != GateKind::And || child2.kind != GateKind::And {
            todo!()
        }
        let (prev, other) = if child1.input1.starts_with('x') && child1.input2.starts_with('y') {
            (child1, child2)
        } else {
            (child2, child1)
        };
        let num = format!("{:02}", i - 1);
        if prev.input1[1..] != num {
            todo!()
        }
        if prev.input2[1..] != num {
            todo!()
        }
        self.check_carry_chain(other, i - 1)
    }

    fn check_carry_chain(&mut self, gate: Gate<'a>, i: usize) -> Option<NoSwap> {
        if i == 1 {
            return Some(NoSwap);
        }
        let child1 = *self.gate_map.get(gate.input1).unwrap();
        let child2 = *self.gate_map.get(gate.input2).unwrap();
        let (term, _other) = if child1.input1.starts_with('x') && child1.input2.starts_with('y') {
            (child1, child2)
        } else {
            (child2, child1)
        };
        if term.kind != GateKind::Xor {
            todo!()
        }
        Some(NoSwap)
    }

    #[allow(dead_code)]
    fn print_tree(&self, out: &'a str, depth: usize) {
        if depth == 0 {
            return;
        }
        let Some(g) = self.gate_map.get(out) else {
            return;
        };
        println!("{g:?}");
        self.print_tree(g.input1, depth - 1);
        self.print_tree(g.input2, depth - 1);
    }

    #[must_use]
    fn add_swap(&mut self, mut a: Gate<'a>, mut b: Gate<'a>) -> Option<NoSwap> {
        self.swaps.push((a.output, b.output));
        std::mem::swap(&mut a.output, &mut b.output);
        self.gate_map.insert(a.output, a);
        self.gate_map.insert(b.output, b);
        None
    }
}

struct NoSwap;

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
                    kind: gate_kind,
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
                        self.states.insert(g.output, g.kind.calc(*v1, *v2));
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
    kind: GateKind,
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

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::PartFn;

pub const PARTS: (PartFn, PartFn) = (part1, part2);

fn part1(input: &str) -> usize {
    let mut config: Configuration = input.parse().unwrap();
    config.prepass();
    config.push_button_n(1000);
    config.count_product()
}

fn part2(input: &str) -> usize {
    let mut config: Configuration = input.parse().unwrap();
    config.prepass();
    let rx_inputs = config.get_inputs("rx");
    assert_eq!(rx_inputs.len(), 1);
    let bc = config.branch_cycle(&rx_inputs[0], &Pulse::Low).period;
    bc
}

fn analyze_cycles(config: Configuration) {
    let mut module_cycles = HashMap::<String, ModuleCycle>::default();
    for (label, m) in config.modules {
        module_cycles.insert(
            label,
            ModuleCycle {
                module: m,
                inputs: Vec::default(),
                cycle: None,
            },
        );
    }

    let mut inputs_map: HashMap<String, Vec<String>> = module_cycles
        .keys()
        .map(|target| {
            (
                target.to_owned(),
                module_cycles
                    .iter()
                    .filter(|(_, m)| m.module.destinations.contains(target))
                    .map(|(l, _)| l.to_owned())
                    .collect(),
            )
        })
        .collect();
    for (label, m) in &mut module_cycles {
        m.inputs = inputs_map.remove(label).unwrap();
    }

    module_cycles.get_mut("broadcaster").unwrap().cycle = Some(1);
    for (_, m) in &module_cycles {
        if m.inputs
            .iter()
            .all(|input| module_cycles.get(input).unwrap().cycle.is_some())
        {
            // dbg!(&m);
        }
    }
    let mut ins = HashSet::new();
    for l in &module_cycles
        .get("broadcaster")
        .unwrap()
        .module
        .destinations
    {
        let m = module_cycles.get(l).unwrap();
        println!("{l} {:?}", m.inputs);
        for d in &m.inputs {
            ins.insert(d);
        }
    }
    for _ in 0..2 {
        println!("--");
        let mut new_ins = HashSet::new();
        for l in ins {
            let m = module_cycles.get(l).unwrap();
            println!("{l} {:?}", m.inputs);
            for d in &m.inputs {
                new_ins.insert(d);
            }
        }
        ins = new_ins;
    }
}

#[derive(Debug)]
struct ModuleCycle {
    module: Module,
    inputs: Vec<String>,
    cycle: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct Configuration {
    modules: HashMap<String, Module>,
    low_count: usize,
    high_count: usize,
}

impl Configuration {
    fn branch_cycle(&self, name: &str, target_value: &Pulse) -> BranchCycle {
        println!("branch cycle {name} {target_value:?}");
        let branch_module_names = self.get_all_deps(name);
        println!("branch deps: {branch_module_names:?}");
        let new_target_value = match self.modules.get(name).unwrap().kind {
            ModuleKind::Broadcaster | ModuleKind::FlipFlop => {
                return self.clone().branch_cycle_brute(name, target_value)
            }
            ModuleKind::Conjunction => target_value.invert(),
        };
        if branch_module_names.contains(name) {
            println!("cyclic deps");
            return self.clone().branch_cycle_brute(name, target_value);
        }
        let input_names = self.get_inputs(name);
        let mut acc = BranchCycle { period: 1 };
        for input_name in &input_names {
            acc = acc.lcm(&self.branch_cycle(input_name, &new_target_value));
        }
        acc
    }

    fn branch_cycle_brute(mut self, name: &str, target_value: &Pulse) -> BranchCycle {
        println!("branch cycle brute {name} {target_value:?}");
        let branch_module_names = self.get_all_deps(name);
        println!("branch deps: {branch_module_names:?}");
        let inital_states = self.get_modules(branch_module_names.iter().map(|x| x.as_str()));
        // let presses = config.push_button_until(branch_target, target_value);
        let mut period = 0;
        let mut offset = None;
        for i in 1..usize::MAX {
            if self.push_button_check_src(name, target_value) {
                println!("FOUND {i}");
                if offset.is_some() {
                    panic!("already found...");
                }
                offset = Some(i);
            }
            if inital_states
                .iter()
                .all(|m| self.modules.get(&m.name).unwrap().implementation == m.implementation)
            {
                println!("period = {}", i);
                period = i;
                break;
            }
        }
        assert_eq!(period, offset.unwrap());
        BranchCycle { period }
    }

    fn count_product(&self) -> usize {
        self.low_count * self.high_count
    }

    fn push_button_n(&mut self, n: usize) {
        for _ in 0..n {
            self.push_button();
        }
    }

    fn push_button_until(&mut self, name: &str, pulse: &Pulse) -> usize {
        for i in 1..usize::MAX {
            if self.push_button_check_dst(name, pulse) {
                return i;
            }
        }
        panic!("did not find match")
    }

    fn start_button() -> PulseWithDestination {
        PulseWithDestination {
            pulse: Pulse::Low,
            src: String::from("button"),
            dst: String::from("broadcaster"),
        }
    }

    fn get_inputs(&self, name: &str) -> Vec<String> {
        self.modules
            .iter()
            .filter(|(_, m)| m.destinations.iter().any(|d| d == name))
            .map(|(l, _)| l.to_owned())
            .collect()
    }

    fn get_all_deps(&self, name: &str) -> HashSet<String> {
        let mut visited = HashSet::new();
        let mut to_visit = Vec::new();
        for (l, _) in self
            .modules
            .iter()
            .filter(|(_, m)| m.destinations.iter().any(|d| d == name))
        {
            to_visit.push(l);
        }
        while let Some(label) = to_visit.pop() {
            if visited.contains(label) {
                continue;
            }
            visited.insert(label.to_owned());
            for (l, _) in self
                .modules
                .iter()
                .filter(|(_, m)| m.destinations.iter().any(|d| d == label))
            {
                to_visit.push(l);
            }
        }
        visited
    }

    fn get_modules<'a>(&self, names: impl Iterator<Item = &'a str>) -> Vec<Module> {
        names
            .map(|name| self.modules.get(name).unwrap().clone())
            .collect()
    }

    pub fn push_button_check_dst(&mut self, name: &str, pulse: &Pulse) -> bool {
        let mut pulses = vec![Self::start_button()];
        let mut found = false;
        while !pulses.is_empty() {
            if !found
                && pulses
                    .iter()
                    .find(|p| &p.pulse == pulse && p.dst == name)
                    .is_some()
            {
                found = true;
            }
            pulses = self.send_pulses(pulses);
        }
        found
    }

    pub fn push_button_check_src(&mut self, name: &str, pulse: &Pulse) -> bool {
        let mut pulses = vec![Self::start_button()];
        let mut found = false;
        while !pulses.is_empty() {
            if !found
                && pulses
                    .iter()
                    .find(|p| &p.pulse == pulse && p.src == name)
                    .is_some()
            {
                found = true;
            }
            pulses = self.send_pulses(pulses);
        }
        found
    }

    pub fn push_button(&mut self) {
        let mut pulses = vec![Self::start_button()];
        while !pulses.is_empty() {
            pulses = self.send_pulses(pulses);
        }
    }

    fn inc_counts(&mut self, pulse: &Pulse) {
        match pulse {
            Pulse::High => self.high_count += 1,
            Pulse::Low => self.low_count += 1,
        }
    }

    pub fn prepass(&mut self) {
        let src_dst_pairs: Vec<(String, String)> = self
            .modules
            .iter()
            .flat_map(|(src, m)| {
                m.destinations
                    .iter()
                    .map(|d| (src.to_owned(), d.to_owned()))
            })
            .collect();
        for (src, dst) in src_dst_pairs {
            if let Some(m) = self.modules.get_mut(&dst) {
                m.prepass(src);
            }
        }
    }

    fn send_pulses(&mut self, pulses: Vec<PulseWithDestination>) -> Vec<PulseWithDestination> {
        pulses
            .into_iter()
            .filter_map(|p| {
                // println!("{} -{:?}-> {}", p.src, p.pulse, p.dst);
                self.inc_counts(&p.pulse);
                let Some(m) = self.modules.get_mut(&p.dst) else {
                    // println!("{} -{:?}-> {}", p.src, p.pulse, p.dst);
                    return None;
                };
                let new_pulse = m.send_pulse(&p.pulse, &p.src)?;
                Some(
                    m.destinations
                        .clone()
                        .into_iter()
                        .map(move |d| PulseWithDestination {
                            pulse: new_pulse,
                            src: p.dst.clone(),
                            dst: d,
                        }),
                )
            })
            .flatten()
            .collect()
    }
}

impl FromStr for Configuration {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let modules = s
            .trim()
            .trim_matches('\n')
            .lines()
            .map(|line| line.parse::<Module>().unwrap())
            .map(|m| (m.name.clone(), m))
            .collect();
        Ok(Self {
            modules,
            low_count: 0,
            high_count: 0,
        })
    }
}

#[derive(Debug, Clone)]
struct Module {
    name: String,
    #[allow(dead_code)]
    kind: ModuleKind,
    implementation: ModuleImplementation,
    destinations: Vec<String>,
}

impl Module {
    fn prepass(&mut self, src: String) {
        self.implementation.prepass(src);
    }

    fn send_pulse(&mut self, pulse: &Pulse, src: &str) -> Option<Pulse> {
        self.implementation.send_pulse(pulse, src)
    }
}

impl FromStr for Module {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once("->").unwrap();
        let name = left.trim();
        let kind = match &name[0..1] {
            "b" if name == "broadcaster" => ModuleKind::Broadcaster,
            "%" => ModuleKind::FlipFlop,
            "&" => ModuleKind::Conjunction,
            _ => panic!("unknown module name: {name}"),
        };
        let implementation = kind.create_implementation();
        let name = if matches!(kind, ModuleKind::Broadcaster) {
            name
        } else {
            &name[1..]
        };
        let name = name.to_owned();
        let destinations = right
            .trim()
            .split(',')
            .map(|x| x.trim().to_owned())
            .collect();
        Ok(Self {
            name,
            kind,
            implementation,
            destinations,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pulse {
    High,
    Low,
}

impl Pulse {
    fn invert(&self) -> Self {
        match self {
            Pulse::High => Pulse::Low,
            Pulse::Low => Pulse::High,
        }
    }
}

#[derive(Debug)]
struct PulseWithDestination {
    pulse: Pulse,
    src: String,
    dst: String,
}

#[derive(Debug, Clone)]
enum ModuleKind {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

impl ModuleKind {
    fn create_implementation(&self) -> ModuleImplementation {
        match self {
            ModuleKind::Broadcaster => ModuleImplementation::Broadcaster(Broadcaster::default()),
            ModuleKind::FlipFlop => ModuleImplementation::FlipFlop(FlipFlop::default()),
            ModuleKind::Conjunction => ModuleImplementation::Conjunction(Conjunction::default()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ModuleImplementation {
    Broadcaster(Broadcaster),
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
}

impl ModuleImplementationTrait for ModuleImplementation {
    fn prepass(&mut self, src: String) {
        match self {
            ModuleImplementation::Broadcaster(x) => x.prepass(src),
            ModuleImplementation::FlipFlop(x) => x.prepass(src),
            ModuleImplementation::Conjunction(x) => x.prepass(src),
        }
    }

    fn send_pulse(&mut self, pulse: &Pulse, src: &str) -> Option<Pulse> {
        match self {
            ModuleImplementation::Broadcaster(x) => x.send_pulse(pulse, src),
            ModuleImplementation::FlipFlop(x) => x.send_pulse(pulse, src),
            ModuleImplementation::Conjunction(x) => x.send_pulse(pulse, src),
        }
    }
}

trait ModuleImplementationTrait: std::fmt::Debug + Eq {
    fn prepass(&mut self, _src: String) {}
    fn send_pulse(&mut self, pulse: &Pulse, src: &str) -> Option<Pulse>;
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Broadcaster;

impl ModuleImplementationTrait for Broadcaster {
    fn send_pulse(&mut self, pulse: &Pulse, _src: &str) -> Option<Pulse> {
        Some(*pulse)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
enum FlipFlop {
    On,
    #[default]
    Off,
}

impl ModuleImplementationTrait for FlipFlop {
    fn send_pulse(&mut self, pulse: &Pulse, _src: &str) -> Option<Pulse> {
        match (pulse, &self) {
            (Pulse::High, _) => None,
            (Pulse::Low, FlipFlop::On) => {
                *self = Self::Off;
                Some(Pulse::Low)
            }
            (Pulse::Low, FlipFlop::Off) => {
                *self = Self::On;
                Some(Pulse::High)
            }
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Conjunction {
    last_input: HashMap<String, Pulse>,
}

impl ModuleImplementationTrait for Conjunction {
    fn prepass(&mut self, src: String) {
        self.last_input.insert(src, Pulse::Low);
    }

    fn send_pulse(&mut self, pulse: &Pulse, src: &str) -> Option<Pulse> {
        self.last_input.insert(src.to_owned(), *pulse);
        if self
            .last_input
            .iter()
            .all(|(_, p)| matches!(p, Pulse::High))
        {
            Some(Pulse::Low)
        } else {
            Some(Pulse::High)
        }
    }
}

struct BranchCycle {
    period: usize,
}

impl BranchCycle {
    fn lcm(&self, other: &Self) -> Self {
        let g = self.gcd(other).period;
        Self {
            period: self.period / g * other.period,
        }
    }

    fn gcd(&self, other: &Self) -> Self {
        let mut a = self.period;
        let mut b = other.period;
        while b != 0 {
            (a, b) = (b, a % b);
        }
        Self { period: a }
    }
}

#[test]
fn example1() {
    let input = "
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
    ";
    assert_eq!(part1(input), 32000000);
}

#[test]
fn example2() {
    let input = "
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
    ";
    assert_eq!(part1(input), 11687500);
}

#[ignore]
#[test]
fn example3() {
    let input = "
broadcaster -> a
%a -> rx
    ";
    assert_eq!(part2(input), 2);
}

#[ignore]
#[test]
fn cycle_measurements() {
    let input = "
broadcaster -> a, inv
%a -> c
&inv -> c
&c -> rx
    ";
    assert_eq!(part2(input), 2);
}

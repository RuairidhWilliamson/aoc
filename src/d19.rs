use std::{collections::HashMap, ops::Range, str::FromStr};

use super::PartFn;

pub const PARTS: (PartFn, PartFn) = (part1, part2);

fn part1(input: &str) -> isize {
    let input = input.parse().unwrap();
    accepted_parts(&input).map(|part| part.sum_ratings()).sum()
}

fn part2(input: &str) -> isize {
    let input: Input = input.parse().unwrap();

    let port_range = PartRange {
        x: 1..4001,
        m: 1..4001,
        a: 1..4001,
        s: 1..4001,
    };
    port_range.run_workflow(&Destination::Workflow(String::from("in")), &input.workflows)
}

fn accepted_parts(input: &Input) -> impl Iterator<Item = &Part> {
    input
        .parts
        .iter()
        .filter(|part| part.is_accepted(&input.workflows))
}

#[derive(Debug, Clone)]
struct Input {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().trim_matches('\n');
        let (workflows, parts) = s.split_once("\n\n").unwrap();
        let workflows = workflows
            .lines()
            .map(|line| {
                let w: Workflow = line.parse().unwrap();
                (w.name.clone(), w)
            })
            .collect();
        let parts = parts.lines().map(|line| line.parse().unwrap()).collect();
        Ok(Self { workflows, parts })
    }
}

#[derive(Debug, Clone)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn find_destination(&self, part: &Part) -> &Destination {
        for rule in &self.rules {
            if let Some(dest) = rule.evaluate(part) {
                return dest;
            }
        }
        panic!("workflow failed to find an matching rule")
    }

    fn find_combinations(
        &self,
        mut part_rng: PartRange,
        workflows: &HashMap<String, Workflow>,
    ) -> isize {
        let mut total = 0;
        for rule in &self.rules {
            if part_rng.is_empty() {
                return 0;
            }
            let (branch, destination, new_part_rng) = rule.segment_rng(part_rng);
            total += branch.run_workflow(destination, workflows);
            part_rng = new_part_rng;
        }
        total
    }
}

impl FromStr for Workflow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, rest) = s.split_once('{').unwrap();
        let name = name.to_owned();
        let rest = rest.strip_suffix('}').unwrap();
        let rules = rest.split(',').map(|rule| rule.parse().unwrap()).collect();
        Ok(Self { name, rules })
    }
}

#[derive(Debug, Clone)]
struct Rule {
    condition: Option<RuleCondition>,
    destination: Destination,
}

impl Rule {
    fn evaluate(&self, part: &Part) -> Option<&Destination> {
        let Some(condition) = &self.condition else {
            return Some(&self.destination);
        };
        if condition.evaluate(part) {
            Some(&self.destination)
        } else {
            None
        }
    }

    fn segment_rng(&self, part_range: PartRange) -> (PartRange, &Destination, PartRange) {
        let Some(condition) = &self.condition else {
            return (part_range, &self.destination, PartRange::new_empty());
        };
        let (a, b) = condition.segment_rng(part_range);
        (a, &self.destination, b)
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((condition, destination)) = s.split_once(':') else {
            let destination = s.parse().unwrap();
            return Ok(Self {
                condition: None,
                destination,
            });
        };

        let condition = Some(condition.parse().unwrap());
        let destination = destination.parse().unwrap();
        Ok(Self {
            condition,
            destination,
        })
    }
}

#[derive(Debug, Clone)]
struct RuleCondition {
    rating: Rating,
    operator: ComparisonOperator,
    value: isize,
}

impl RuleCondition {
    fn evaluate(&self, part: &Part) -> bool {
        self.operator
            .compare(part.get_rating(&self.rating), self.value)
    }

    fn segment_rng(&self, mut part_range: PartRange) -> (PartRange, PartRange) {
        let rng = part_range.get_rating(&self.rating);
        let (a, b) = self.operator.segment(rng, self.value);
        let mut branch = part_range.clone();
        *branch.get_rating_mut(&self.rating) = a;
        *part_range.get_rating_mut(&self.rating) = b;
        (branch, part_range)
    }
}

impl FromStr for RuleCondition {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rating = s[0..1].parse().unwrap();
        let operator = s[1..2].parse().unwrap();
        let value = s[2..].parse().unwrap();
        Ok(Self {
            rating,
            operator,
            value,
        })
    }
}

#[derive(Debug, Clone)]
enum Destination {
    Reject,
    Accept,
    Workflow(String),
}

impl FromStr for Destination {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Self::Reject),
            "A" => Ok(Self::Accept),
            label => Ok(Self::Workflow(label.to_owned())),
        }
    }
}

#[derive(Debug, Clone)]
struct Part {
    x: isize,
    m: isize,
    a: isize,
    s: isize,
}

impl Part {
    fn sum_ratings(&self) -> isize {
        self.x + self.m + self.a + self.s
    }

    fn is_accepted(&self, workflows: &HashMap<String, Workflow>) -> bool {
        let mut label = "in";
        loop {
            let w = workflows.get(label).unwrap();
            let dest = w.find_destination(self);
            match dest {
                Destination::Reject => return false,
                Destination::Accept => return true,
                Destination::Workflow(new_label) => {
                    label = new_label;
                }
            }
        }
    }

    fn get_rating(&self, rating: &Rating) -> isize {
        match rating {
            Rating::X => self.x,
            Rating::M => self.m,
            Rating::A => self.a,
            Rating::S => self.s,
        }
    }
}

impl FromStr for Part {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix('{').unwrap().strip_suffix('}').unwrap();
        let mut p = Part {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        };
        for x in s.split(',') {
            let r: Rating = x[0..1].parse().unwrap();
            let v: isize = x[2..].parse().unwrap();
            match r {
                Rating::X => {
                    p.x = v;
                }
                Rating::M => {
                    p.m = v;
                }
                Rating::A => {
                    p.a = v;
                }
                Rating::S => {
                    p.s = v;
                }
            }
        }
        Ok(p)
    }
}

#[derive(Debug, Clone)]
enum Rating {
    X,
    M,
    A,
    S,
}

impl FromStr for Rating {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Self::X),
            "m" => Ok(Self::M),
            "a" => Ok(Self::A),
            "s" => Ok(Self::S),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
enum ComparisonOperator {
    GreaterThan,
    LessThan,
}

impl ComparisonOperator {
    fn compare(&self, a: isize, b: isize) -> bool {
        match self {
            ComparisonOperator::GreaterThan => a > b,
            ComparisonOperator::LessThan => a < b,
        }
    }

    fn segment(&self, rng: &Range<isize>, value: isize) -> (Range<isize>, Range<isize>) {
        match self {
            ComparisonOperator::GreaterThan => (
                (value + 1).max(rng.start)..rng.end,
                rng.start..(value + 1).min(rng.end),
            ),
            ComparisonOperator::LessThan => (
                rng.start..(value).min(rng.end),
                (value).max(rng.start)..rng.end,
            ),
        }
    }
}

impl FromStr for ComparisonOperator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ">" => Ok(Self::GreaterThan),
            "<" => Ok(Self::LessThan),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
struct PartRange {
    x: Range<isize>,
    m: Range<isize>,
    a: Range<isize>,
    s: Range<isize>,
}

impl PartRange {
    fn new_empty() -> Self {
        Self {
            x: 0..0,
            m: 0..0,
            a: 0..0,
            s: 0..0,
        }
    }

    fn is_empty(&self) -> bool {
        self.x.is_empty() && self.m.is_empty() && self.a.is_empty() && self.s.is_empty()
    }

    fn get_rating(&self, rating: &Rating) -> &Range<isize> {
        match rating {
            Rating::X => &self.x,
            Rating::M => &self.m,
            Rating::A => &self.a,
            Rating::S => &self.s,
        }
    }

    fn get_rating_mut(&mut self, rating: &Rating) -> &mut Range<isize> {
        match rating {
            Rating::X => &mut self.x,
            Rating::M => &mut self.m,
            Rating::A => &mut self.a,
            Rating::S => &mut self.s,
        }
    }

    fn distinct_combinations(&self) -> isize {
        lenrng(&self.x) * lenrng(&self.m) * lenrng(&self.a) * lenrng(&self.s)
    }

    fn run_workflow(
        self,
        destination: &Destination,
        workflows: &HashMap<String, Workflow>,
    ) -> isize {
        match destination {
            Destination::Reject => 0,
            Destination::Accept => self.distinct_combinations(),
            Destination::Workflow(label) => {
                let w = workflows.get(label).unwrap();
                w.find_combinations(self, workflows)
            }
        }
    }
}

fn lenrng(rng: &Range<isize>) -> isize {
    rng.end - rng.start
}

#[test]
fn example1() {
    let input = "
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
    ";

    assert_eq!(part1(input), 19114);
    assert_eq!(part2(input), 167409079868000);
}

#[test]
fn test_comparison_segment() {
    let (b, a) = ComparisonOperator::GreaterThan.segment(&(0..10), 4);
    assert_eq!(a, 0..5);
    assert_eq!(b, 5..10);
    let (b, a) = ComparisonOperator::GreaterThan.segment(&(0..10), 11);
    assert_eq!(a, 0..10);
    assert!(b.is_empty());
    let (b, a) = ComparisonOperator::GreaterThan.segment(&(0..10), -5);
    assert!(a.is_empty());
    assert_eq!(b, 0..10);
    let (b, a) = ComparisonOperator::GreaterThan.segment(&(0..10), -1);
    assert!(a.is_empty(), "{a:?} should be empty");
    assert_eq!(b, 0..10);
    let (b, a) = ComparisonOperator::GreaterThan.segment(&(0..10), 9);
    assert_eq!(a, 0..10);
    assert!(b.is_empty());

    let (a, b) = ComparisonOperator::LessThan.segment(&(0..10), 4);
    assert_eq!(a, 0..4);
    assert_eq!(b, 4..10);
    let (a, b) = ComparisonOperator::LessThan.segment(&(0..10), 0);
    assert!(a.is_empty());
    assert_eq!(b, 0..10);
}

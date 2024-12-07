pub fn solve_part1(input: &str) -> usize {
    Equation::parse_lines(input)
        .filter(|e| e.has_solution())
        .map(|e| e.target)
        .sum()
}

pub fn solve_part2(input: &str) -> usize {
    Equation::parse_lines(input)
        .filter(|e| e.has_solution2())
        .map(|e| e.target)
        .sum()
}

struct Equation {
    target: usize,
    args: Vec<usize>,
}

impl Equation {
    fn parse_lines(input: &str) -> impl Iterator<Item = Self> + use<'_> {
        input.lines().map(|l| Self::parse(l))
    }

    fn parse(line: &str) -> Self {
        let (target, args) = line.split_once(": ").unwrap();
        let target: usize = target.parse().unwrap();
        let args = args.split(' ').map(|a| a.parse().unwrap()).collect();
        Self { target, args }
    }

    fn calc(&self, oc: OperatorChain) -> usize {
        let mut args = self.args.iter();
        let mut value = *args.next().unwrap();
        for (a, o) in args.zip(oc) {
            match o {
                Operator::Add => {
                    value += a;
                }
                Operator::Mul => {
                    value *= a;
                }
            }
        }
        value
    }

    fn has_solution(&self) -> bool {
        (0..(1 << self.args.len())).any(|oc| self.calc(OperatorChain(oc)) == self.target)
    }

    fn calc2(&self, index: usize, current: usize) -> bool {
        if index == self.args.len() {
            return current == self.target;
        }
        self.calc2(index + 1, Operator2::Add.eval(current, self.args[index]))
            || self.calc2(index + 1, Operator2::Mul.eval(current, self.args[index]))
            || self.calc2(index + 1, Operator2::Con.eval(current, self.args[index]))
    }

    fn has_solution2(&self) -> bool {
        self.calc2(1, self.args[0])
    }
}

struct OperatorChain(u64);

impl Iterator for OperatorChain {
    type Item = Operator;

    fn next(&mut self) -> Option<Self::Item> {
        let out = match self.0 & 1 {
            0 => Operator::Add,
            1 => Operator::Mul,
            _ => unreachable!(),
        };
        self.0 = self.0 >> 1;
        Some(out)
    }
}

enum Operator {
    Add,
    Mul,
}

enum Operator2 {
    Add,
    Mul,
    Con,
}

impl Operator2 {
    fn eval(&self, lhs: usize, rhs: usize) -> usize {
        match self {
            Operator2::Add => lhs + rhs,
            Operator2::Mul => lhs * rhs,
            Operator2::Con => lhs * 10_usize.pow(rhs.ilog10() + 1) + rhs,
        }
    }
}

#[cfg(test)]
const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

#[test]
fn practice_part1() {
    assert_eq!(solve_part1(INPUT), 3749);
}

#[test]
fn practice_part2() {
    assert_eq!(solve_part2(INPUT), 11387);
}

#[test]
fn concat() {
    assert_eq!(Operator2::Con.eval(539, 456), 539456);
}

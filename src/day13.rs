use aoc_helper::grid::Vec2;

pub fn solve_part1(input: &str) -> usize {
    VendingMachine::parse(input)
        .filter_map(|v| v.solve_button_presses())
        .map(VendingMachine::button_cost)
        .sum()
}

pub fn solve_part2(input: &str) -> usize {
    VendingMachine::parse(input)
        .map(|mut v| {
            v.prize += Vec2::new(10000000000000, 10000000000000);
            v
        })
        .filter_map(|v| v.solve_button_presses())
        .map(VendingMachine::button_cost)
        .sum()
}

#[derive(Debug, PartialEq, Eq)]
struct VendingMachine {
    button_a: Vec2,
    button_b: Vec2,
    prize: Vec2,
}

impl VendingMachine {
    fn parse(input: &str) -> impl Iterator<Item = Self> + use<'_> {
        input.split("\n\n").map(|v_input| {
            let mut lines = v_input.lines();
            let button_a = Self::parse_button_line(lines.next().unwrap(), "Button A");
            let button_b = Self::parse_button_line(lines.next().unwrap(), "Button B");
            let prize = Self::parse_prize_line(lines.next().unwrap());
            Self {
                button_a,
                button_b,
                prize,
            }
        })
    }

    fn parse_button_line(line: &str, expected_lhs: &str) -> Vec2 {
        let (lhs, rhs) = line.split_once(": ").unwrap();
        debug_assert_eq!(lhs, expected_lhs);
        let (x, y) = rhs.split_once(", ").unwrap();
        let (x_str, x) = x.split_once('+').unwrap();
        debug_assert_eq!(x_str, "X");
        let x = x.parse().unwrap();
        let (y_str, y) = y.split_once('+').unwrap();
        debug_assert_eq!(y_str, "Y");
        let y = y.parse().unwrap();
        Vec2::new(x, y)
    }

    fn parse_prize_line(line: &str) -> Vec2 {
        let (lhs, rhs) = line.split_once(": ").unwrap();
        debug_assert_eq!(lhs, "Prize");
        let (x, y) = rhs.split_once(", ").unwrap();
        let (x_str, x) = x.split_once('=').unwrap();
        debug_assert_eq!(x_str, "X");
        let x = x.parse().unwrap();
        let (y_str, y) = y.split_once('=').unwrap();
        debug_assert_eq!(y_str, "Y");
        let y = y.parse().unwrap();
        Vec2::new(x, y)
    }

    fn solve_button_presses(&self) -> Option<(usize, usize)> {
        let num = self.prize.y * self.button_a.x - self.prize.x * self.button_a.y;
        let denom = self.button_b.y * self.button_a.x - self.button_b.x * self.button_a.y;
        if num.rem_euclid(denom) != 0 {
            return None;
        }
        let b = num / denom;
        let num = self.prize - self.button_b * b;
        let denom = self.button_a;
        let rem = num.rem_euclid(denom);
        if rem.x != 0 || rem.y != 0 {
            return None;
        }
        let a = num / denom;
        if a.x != a.y {
            panic!("x did not equal y");
        }
        let a = a.x;
        if a < 0 || b < 0 {
            return None;
        }
        debug_assert_eq!(self.button_a * a + self.button_b * b, self.prize);
        Some((a as usize, b as usize))
    }

    fn button_cost((a_presses, b_presses): (usize, usize)) -> usize {
        a_presses * 3 + b_presses
    }
}

#[cfg(test)]
const INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

#[test]
fn practice_part1() {
    assert_eq!(solve_part1(INPUT), 480);
}

#[test]
fn practice_part2() {
    let solvable: Vec<_> = VendingMachine::parse(INPUT)
        .map(|mut v| {
            v.prize += Vec2::new(10000000000000, 10000000000000);
            v
        })
        .map(|v| v.solve_button_presses().is_some())
        .collect();
    assert_eq!(solvable, vec![false, true, false, true]);
}

#[test]
fn part2_parse_check() {
    let v: Vec<_> = VendingMachine::parse(
        "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=10000000008400, Y=10000000005400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=10000000012748, Y=10000000012176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=10000000007870, Y=10000000006450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=10000000018641, Y=10000000010279
",
    )
    .collect();
    assert_eq!(
        VendingMachine::parse(INPUT)
            .map(|mut v| {
                v.prize += Vec2::new(10000000000000, 10000000000000);
                v
            })
            .collect::<Vec<_>>(),
        v
    );
}

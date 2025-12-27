use crate::{
    grid::{AugmentedMatrix, Grid},
    magic_iter::MagicIterVec,
};

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(Machine::parse_from_line)
        .map(|m| m.fewest_presses())
        .sum()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(Problem::parse_line)
        .map(|problem| {
            let answer = problem.solve();
            println!("{answer:?}");
            answer.iter().copied().sum::<isize>() as usize
        })
        .sum()
}

struct Machine {
    goal: State,
    buttons: Vec<State>,
}

impl Machine {
    fn parse_from_line(line: &str) -> Self {
        let (goal, rest) = line.split_once(' ').unwrap();
        let goal = State::parse_from_goal(goal);

        let (rest, _joltage) = rest.rsplit_once(' ').unwrap();

        let buttons = rest.split(' ').map(State::parse_from_button).collect();
        Self { goal, buttons }
    }

    fn fewest_presses(&self) -> u32 {
        (0usize..(1 << self.buttons.len()))
            .filter(|c| self.press(*c) == self.goal)
            .map(usize::count_ones)
            .min()
            .unwrap()
    }

    fn press(&self, mut press: usize) -> State {
        let mut state = State::default();
        let mut i = 0;
        while press > 0 && i < self.buttons.len() {
            if press & 1 == 1 {
                state = state.press(self.buttons[i]);
            }
            press >>= 1;
            i += 1;
        }
        state
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct State(usize);

impl State {
    fn parse_from_goal(mut goal: &str) -> Self {
        goal = goal.strip_prefix('[').unwrap().strip_suffix(']').unwrap();
        let mut out = 0;
        for (i, c) in goal.chars().enumerate() {
            let v = match c {
                '.' => 0,
                '#' => 1,
                _ => panic!(),
            };
            out += v << i;
        }
        Self(out)
    }

    fn parse_from_button(mut button: &str) -> Self {
        button = button.strip_prefix('(').unwrap().strip_suffix(')').unwrap();
        let mut out = 0;
        for b in button.split(',') {
            let index: usize = b.parse().unwrap();
            out += 1 << index;
        }
        Self(out)
    }

    fn press(self, s: Self) -> Self {
        Self(self.0 ^ s.0)
    }
}

struct Problem {
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

impl Problem {
    fn parse_line(line: &str) -> Self {
        println!("{line}");
        let (_goal, rest) = line.split_once(' ').unwrap();
        let (rest, joltage) = rest.rsplit_once(' ').unwrap();
        let joltage: Vec<_> = joltage
            .strip_prefix('{')
            .unwrap()
            .strip_suffix('}')
            .unwrap()
            .split(',')
            .map(|j| j.parse().unwrap())
            .collect();
        let buttons: Vec<Vec<_>> = rest
            .trim()
            .split(' ')
            .map(|buttons| {
                buttons
                    .trim()
                    .strip_prefix('(')
                    .unwrap()
                    .strip_suffix(')')
                    .unwrap()
                    .split(',')
                    .map(|b| b.parse().unwrap())
                    .collect()
            })
            .collect();
        Self { buttons, joltage }
    }

    fn check_solution_signed(&self, presses: impl Clone + Iterator<Item = isize>) -> bool {
        if !presses.clone().all(|x| x >= 0) {
            return false;
        }
        (0..self.joltage.len()).all(|i| {
            let sum = presses
                .clone()
                .zip(self.buttons.iter())
                .filter(|(_, b)| b.contains(&i))
                .map(|(p, _)| p)
                .sum::<isize>() as usize;
            let j = self.joltage[i];
            if sum != j {
                return false;
            }
            true
        })
    }

    fn solve(&self) -> Vec<isize> {
        struct Solution {
            answer: Vec<isize>,
            sum: isize,
            missing_sum: usize,
        }
        let mut matrix = self.build_matrix_new();
        matrix.bareiss();
        matrix.remove_trailing_zero_rows();
        let mut best: Option<Solution> = None;
        let mut largest_missing = 0;
        let missing = (matrix.width() - 1).saturating_sub(matrix.rank());
        let mut iter = MagicIterVec::new(missing);
        while iter.update_next() {
            let missing_values = iter.state();
            if largest_missing < iter.monotonic_sum() {
                if iter.monotonic_sum() > self.max_joltage() {
                    break;
                }
                if let Some(best) = best.as_ref() {
                    if best.missing_sum + 10 < iter.monotonic_sum() {
                        // break once the monotonic sum increases and we are not finding solutions
                        break;
                    }
                }
                largest_missing = iter.monotonic_sum();
            }
            let Some(answer) = matrix.solve_with_unknowns(&missing_values) else {
                continue;
            };
            if !self.check_solution_signed(answer.iter().copied()) {
                continue;
            }
            let sum: isize = answer.iter().copied().sum();
            if best.as_ref().is_none_or(|best| sum < best.sum) {
                best = Some(Solution {
                    answer,
                    sum: sum,
                    missing_sum: iter.monotonic_sum(),
                });
            }
        }
        best.unwrap().answer
    }

    fn max_joltage(&self) -> usize {
        self.joltage.iter().copied().max().unwrap()
    }

    fn build_matrix_new(&self) -> AugmentedMatrix<isize> {
        let mut matrix = Grid::<isize>::new_fill(0, self.buttons.len() + 1, self.joltage.len());
        for (i, buttons) in self.buttons.iter().enumerate() {
            for b in buttons {
                matrix[(i, *b)] = 1;
            }
        }
        for (i, x) in self.joltage.iter().enumerate() {
            matrix[(self.buttons.len(), i)] = *x as isize;
        }
        AugmentedMatrix(matrix)
    }
}

#[test]
fn try_part2_integer() {
    let input = "[#####.###.] (4,7,8) (0,1,2,3,5,6,8,9) (0,4,5,7,8,9) (2,3,5) (0,2,3,4,5,6,7,8) (5,6) (0,1,2,3,4,5,9) (0,1,2,5,6,9) (0,3,4,5,6,7,8,9) (3,4,5,6,8) (0,1,2,3,4,5,6,7,9) (0,8) (3,4,8,9) {261,225,243,252,56,278,262,29,257,242}";
    let problem = Problem::parse_line(input);
    let mut matrix = problem.build_matrix_new();
    assert!(matrix.check_solution(&[0, 203, 7, 0, 18, 12, 9, 13, 4, 12, 0, 7, 6]));
    println!("{}", matrix.display());
    matrix.bareiss();
    println!("{}", matrix.display());
    let solution = matrix.solve_with_unknowns(&[0, 7, 6]).unwrap();
    assert!(problem.check_solution_signed(solution.iter().copied()));
}

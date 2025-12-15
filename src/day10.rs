use itertools::{Itertools as _, repeat_n};
use rayon::iter::{IndexedParallelIterator as _, IntoParallelIterator as _, ParallelIterator as _};

use crate::grid::Grid;

const FUDGE_TOLERANCE: f32 = 0.01;

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
        .map(|problem| problem.solve_please())
        .sum()
}

struct Problem {
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

impl Problem {
    fn parse_line(line: &str) -> Self {
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

    fn check_solution_unsigned(&self, presses: &[usize]) -> bool {
        (0..self.joltage.len()).all(|i| {
            let sum = presses
                .iter()
                .zip(self.buttons.iter())
                .filter(|(_, b)| b.contains(&i))
                .map(|(p, _)| p)
                .sum::<usize>() as usize;
            let j = self.joltage[i];
            if sum != j {
                return false;
            }
            true
        })
    }

    fn solve_please(&self) -> usize {
        let matrix = self.build_matrix();
        let a_bad_solution = self.find_any_solution(&matrix).unwrap();
        assert!(self.check_solution_unsigned(&a_bad_solution));
        let total = a_bad_solution.iter().sum::<usize>();
        let max = self.joltage.iter().copied().max().unwrap();
        if let Some(solution) = (max..total)
            .into_par_iter()
            .rev()
            .find_map_last(|n| self.can_solve_n(&matrix, n as f32))
        {
            assert!(
                self.check_solution_unsigned(&solution),
                "found a bad solution"
            );
            solution.iter().sum()
        } else {
            total
        }
    }

    fn build_matrix(&self) -> Grid<f32> {
        let mut matrix = Grid::<f32>::new_fill(0.0, self.buttons.len() + 1, self.joltage.len());
        for (i, buttons) in self.buttons.iter().enumerate() {
            for b in buttons {
                matrix[(i, *b)] = 1.0;
            }
        }
        for (i, x) in self.joltage.iter().enumerate() {
            matrix[(self.buttons.len(), i)] = *x as f32;
        }
        matrix.guassian_elimination();
        matrix.reduced_row_echelon();
        matrix
    }

    fn find_any_solution(&self, matrix: &Grid<f32>) -> Option<Vec<usize>> {
        self.search_indeteriminate_matrix_solutions(matrix)
    }

    fn can_solve_n(&self, matrix: &Grid<f32>, n: f32) -> Option<Vec<usize>> {
        let mut matrix = matrix.clone();
        let mut v = vec![1.0; matrix.width()];
        v[matrix.width() - 1] = n;
        matrix.add_row(&v);
        self.search_indeteriminate_matrix_solutions(&matrix)
    }

    fn search_indeteriminate_matrix_solutions(&self, matrix: &Grid<f32>) -> Option<Vec<usize>> {
        let mut matrix = matrix.clone();
        let v = vec![0.0; matrix.width()];
        while matrix.height() < matrix.width() - 1 {
            matrix.add_row(&v);
        }
        matrix.guassian_elimination();
        matrix.reduced_row_echelon();
        let max = self.joltage.iter().sum::<usize>() as usize;
        let mut missing = 0;
        {
            let mut matrix = matrix.clone();
            'outer: loop {
                for i in 0..matrix.width() - 1 {
                    if (matrix[(i, i)] - 1.0).abs() > f32::EPSILON {
                        let mut v = vec![0.0; matrix.width()];
                        v[i] = 1.0;
                        v[matrix.width() - 1] = 6.0;
                        matrix.add_row(&v);
                        matrix.guassian_elimination();
                        matrix.reduced_row_echelon();
                        missing += 1;
                        continue 'outer;
                    }
                }
                break;
            }
        }
        'next: for indeterminates in repeat_n(0..=max, missing).multi_cartesian_product() {
            let mut matrix = matrix.clone();
            let mut missing = 0;
            'outer: loop {
                for i in 0..matrix.width() - 1 {
                    if (matrix[(i, i)] - 1.0).abs() > f32::EPSILON {
                        let mut v = vec![0.0; matrix.width()];
                        v[i] = 1.0;
                        v[matrix.width() - 1] = indeterminates[missing] as f32;
                        matrix.add_row(&v);
                        matrix.guassian_elimination();
                        matrix.reduced_row_echelon();
                        missing += 1;
                        continue 'outer;
                    }
                }
                break;
            }
            let mut answer = vec![0.0; matrix.width() - 1];
            for i in (0..matrix.width() - 1).rev() {
                let s: f32 = (i..matrix.width() - 1)
                    .map(|j| answer[j] * matrix[(j, i)])
                    .sum();
                let value = (matrix[(matrix.width() - 1, i)] - s) / matrix[(i, i)];
                let rounded_value = value.round();
                if value < -0.01 || (rounded_value - value).abs() > FUDGE_TOLERANCE {
                    continue 'next;
                }
                answer[i] = rounded_value;
            }
            #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
            let answer: Vec<_> = answer.into_iter().map(|b| b as usize).collect();
            if self.check_solution_unsigned(&answer) {
                return Some(answer);
            }
        }
        None
    }
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

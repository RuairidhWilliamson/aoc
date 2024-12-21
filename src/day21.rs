use std::collections::{BinaryHeap, HashMap};

use aoc_helper::grid::{Grid, Vec2};

pub fn solve_part1(input: &str) -> usize {
    let keypads = Keypads::new();
    input
        .lines()
        .map(|l| {
            let num: usize = l[..l.len() - 1].parse().unwrap();
            let length = keypads.search_shortest::<2>(parse_numerical_code(l).collect());
            println!("{length}");
            num * length
        })
        .sum()
}

pub fn solve_part2(input: &str) -> usize {
    let keypads = Keypads::new();
    input
        .lines()
        .map(|l| {
            let num: usize = l[..l.len() - 1].parse().unwrap();
            let length = keypads.search_shortest::<25>(parse_numerical_code(l).collect());
            println!("{length}");
            num * length
        })
        .sum()
}

fn parse_numerical_code(l: &str) -> impl Iterator<Item = Numerical> + use<'_> {
    l.chars().map(Numerical::parse)
}

impl Keypads {
    fn search_shortest<const N: usize>(&self, code: Vec<Numerical>) -> usize {
        let mut total = 0;
        for i in 0..code.len() {
            let c = *code.get(i).unwrap();
            let prev = code
                .get(i.wrapping_sub(1))
                .copied()
                .unwrap_or(Numerical::Press);
            let state = State::<N> {
                numerical_robot: self.find_numerical_pos(prev),
                directional_robots: [self.find_directional_pos(Directional::Press); N],
            };
            let length = self.search_shortest_single_digit(state, c);
            total += length;
        }
        total
    }

    fn search_shortest_single_digit<const N: usize>(
        &self,
        init: State<N>,
        expected_numerical: Numerical,
    ) -> usize {
        let mut open = BinaryHeap::new();
        open.push(StateWithF { state: init, f: 0 });
        let mut g_map = HashMap::new();
        g_map.insert(init, 0);
        while let Some(StateWithF { state: q, f: _ }) = open.pop() {
            let g = g_map.get(&q).unwrap();
            let new_g = g + 1;
            for d in [
                Directional::Up,
                Directional::Left,
                Directional::Down,
                Directional::Right,
                Directional::Press,
            ] {
                if let Some(apply) = q.apply(self, d, expected_numerical) {
                    let new_q = match apply {
                        Apply::NewState(new_q) => new_q,
                        Apply::PressNumerical => {
                            return new_g;
                        }
                    };
                    let old_g = g_map.entry(new_q).or_insert(usize::MAX);
                    if *old_g > new_g {
                        *old_g = new_g;
                        let h = new_q.heurisitc(self, expected_numerical);
                        let f = new_g + h;
                        open.push(StateWithF { state: new_q, f });
                    }
                }
            }
        }
        panic!("did not find solution")
    }

    fn find_numerical_pos(&self, numerical: Numerical) -> Vec2 {
        self.numerical
            .coords_iter()
            .find(|c| self.numerical.get(*c).unwrap() == &Some(numerical))
            .unwrap()
    }

    fn find_directional_pos(&self, directional: Directional) -> Vec2 {
        self.directional
            .coords_iter()
            .find(|c| self.directional.get(*c).unwrap() == &Some(directional))
            .unwrap()
    }
}

struct StateWithF<const N: usize> {
    state: State<N>,
    f: usize,
}

impl<const N: usize> PartialEq for StateWithF<N> {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state && self.f == other.f
    }
}

impl<const N: usize> Eq for StateWithF<N> {}

impl<const N: usize> PartialOrd for StateWithF<N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> Ord for StateWithF<N> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.f
            .cmp(&other.f)
            .reverse()
            .then(self.state.cmp(&other.state))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct State<const N: usize> {
    numerical_robot: Vec2,
    directional_robots: [Vec2; N],
}

impl<const N: usize> State<N> {
    fn heurisitc(&self, keypads: &Keypads, expected_numerical: Numerical) -> usize {
        let target = keypads
            .numerical
            .coords_iter()
            .find(|c| *keypads.numerical.get(*c).unwrap() == Some(expected_numerical))
            .unwrap();
        1000 - (self.numerical_robot - target).l1_norm()
    }

    fn apply(
        &self,
        keypads: &Keypads,
        mut d: Directional,
        expected_numerical: Numerical,
    ) -> Option<Apply<N>> {
        let mut q: Self = *self;
        for r in &mut q.directional_robots {
            match d {
                Directional::Up => {
                    *r += Vec2::new(0, -1);
                    keypads.directional.get(*r)?.as_ref()?;
                    return Some(Apply::NewState(q));
                }
                Directional::Down => {
                    *r += Vec2::new(0, 1);
                    keypads.directional.get(*r)?.as_ref()?;
                    return Some(Apply::NewState(q));
                }
                Directional::Left => {
                    *r += Vec2::new(-1, 0);
                    keypads.directional.get(*r)?.as_ref()?;
                    return Some(Apply::NewState(q));
                }
                Directional::Right => {
                    *r += Vec2::new(1, 0);
                    keypads.directional.get(*r)?.as_ref()?;
                    return Some(Apply::NewState(q));
                }
                Directional::Press => {
                    d = *keypads.directional.get(*r)?.as_ref()?;
                }
            }
        }
        let r = &mut q.numerical_robot;
        match d {
            Directional::Up => {
                *r += Vec2::new(0, -1);
                keypads.numerical.get(*r)?.as_ref()?;
                Some(Apply::NewState(q))
            }
            Directional::Down => {
                *r += Vec2::new(0, 1);
                keypads.numerical.get(*r)?.as_ref()?;
                Some(Apply::NewState(q))
            }
            Directional::Left => {
                *r += Vec2::new(-1, 0);
                keypads.numerical.get(*r)?.as_ref()?;
                Some(Apply::NewState(q))
            }
            Directional::Right => {
                *r += Vec2::new(1, 0);
                keypads.numerical.get(*r)?.as_ref()?;
                Some(Apply::NewState(q))
            }
            Directional::Press => {
                let n = *keypads.numerical.get(*r)?.as_ref()?;
                if n != expected_numerical {
                    None
                } else {
                    Some(Apply::PressNumerical)
                }
            }
        }
    }
}

enum Apply<const N: usize> {
    NewState(State<N>),
    PressNumerical,
}

struct Keypads {
    numerical: Grid<Option<Numerical>>,
    directional: Grid<Option<Directional>>,
}

impl Keypads {
    fn new() -> Self {
        Self {
            numerical: numerical_keypad(),
            directional: directional_keypad(),
        }
    }
}

type DirectionalKeypad = Grid<Option<Directional>>;

fn directional_keypad() -> DirectionalKeypad {
    Grid::new(
        vec![
            None,
            Some(Directional::Up),
            Some(Directional::Press),
            Some(Directional::Left),
            Some(Directional::Down),
            Some(Directional::Right),
        ]
        .into(),
        3,
    )
}

fn numerical_keypad() -> Grid<Option<Numerical>> {
    Grid::new(
        vec![
            Some(Numerical::Seven),
            Some(Numerical::Eight),
            Some(Numerical::Nine),
            Some(Numerical::Four),
            Some(Numerical::Five),
            Some(Numerical::Six),
            Some(Numerical::One),
            Some(Numerical::Two),
            Some(Numerical::Three),
            None,
            Some(Numerical::Zero),
            Some(Numerical::Press),
        ]
        .into(),
        3,
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Directional {
    Up,
    Down,
    Left,
    Right,
    Press,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Numerical {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Press,
}

impl Numerical {
    fn parse(c: char) -> Self {
        match c {
            '0' => Self::Zero,
            '1' => Self::One,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'A' => Self::Press,
            _ => panic!("unexpected char {c}"),
        }
    }
}

#[cfg(test)]
const INPUT: &str = "029A
980A
179A
456A
379A";

#[test]
fn press_example() {
    assert_eq!(
        Keypads::new().search_shortest::<2>(parse_numerical_code("029A").collect()),
        68
    );
}

#[test]
fn practice_part1() {
    assert_eq!(solve_part1(INPUT), 126384);
}

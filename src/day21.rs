use std::collections::{BinaryHeap, HashMap};

use aoc_helper::grid::{Grid, Vec2};

pub fn solve_part1(input: &str) -> usize {
    solve_puzzle(input, 2)
}

pub fn solve_part2(input: &str) -> usize {
    solve_puzzle(input, 25)
}

fn parse_numerical_code(l: &str) -> impl Iterator<Item = Numerical> + use<'_> {
    l.chars().map(Numerical::parse)
}

fn solve_puzzle(input: &str, n: usize) -> usize {
    let keypads = Keypads::new();
    let cost_map = CostMap::base().next_n(&keypads, n);
    input
        .lines()
        .map(|l| {
            let num: usize = l[..l.len() - 1].parse().unwrap();
            let code: Vec<Numerical> = parse_numerical_code(l).collect();
            let mut total = 0;
            let mut from = Numerical::Press;
            for to in code {
                total += cost_map.shortest_numerical(&keypads, from, to);
                from = to;
            }
            num * total
        })
        .sum()
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Directional {
    Up,
    Down,
    Left,
    Right,
    Press,
}

impl Directional {
    fn variants_as_array() -> [Self; 5] {
        [Self::Up, Self::Down, Self::Left, Self::Right, Self::Press]
    }

    fn as_vec2(&self) -> Vec2 {
        match self {
            Directional::Up => Vec2::new(0, -1),
            Directional::Down => Vec2::new(0, 1),
            Directional::Left => Vec2::new(-1, 0),
            Directional::Right => Vec2::new(1, 0),
            Directional::Press => panic!(),
        }
    }
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

#[derive(Debug)]
struct CostMap(HashMap<(Directional, Directional), usize>);

impl CostMap {
    fn base() -> CostMap {
        Self(
            Directional::variants_as_array()
                .into_iter()
                .flat_map(|d1| Directional::variants_as_array().map(|d2| (d1, d2)))
                .map(|d| (d, 1))
                .collect(),
        )
    }

    fn next(&self, keypads: &Keypads) -> CostMap {
        Self(
            Directional::variants_as_array()
                .into_iter()
                .flat_map(|d1| {
                    Directional::variants_as_array().map(|d2| {
                        let pair = (d1, d2);
                        let cost = self.shortest(keypads, d1, d2);
                        (pair, cost)
                    })
                })
                .collect(),
        )
    }

    fn next_n(&self, keypads: &Keypads, n: usize) -> CostMap {
        let mut s = self.next(keypads);
        for _ in 1..n {
            s = s.next(keypads);
        }
        s
    }

    fn get(&self, current: Directional, d: Directional) -> usize {
        *self.0.get(&(current, d)).unwrap()
    }

    fn get_chain_p_p(&self, ds: &[Directional]) -> usize {
        let mut prev = &Directional::Press;
        let mut total = 0;
        for d in ds {
            total += self.get(*prev, *d);
            prev = d;
        }
        total + self.get(*prev, Directional::Press)
    }

    fn shortest(&self, keypads: &Keypads, from: Directional, to: Directional) -> usize {
        if from == to {
            return self.get(Directional::Press, Directional::Press);
        }
        let from_pos = keypads.find_directional_pos(from);
        let to_pos = keypads.find_directional_pos(to);
        let del = to_pos - from_pos;
        use Directional::*;

        match (del.x, del.y) {
            (0, 0) => self.get_chain_p_p(&[]),
            (1, 0) => self.get_chain_p_p(&[Right]),
            (0, -1) => self.get_chain_p_p(&[Up]),
            (-1, 0) => self.get_chain_p_p(&[Left]),
            (0, 1) => self.get_chain_p_p(&[Down]),
            (1, -1) if from == Left => self.get_chain_p_p(&[Right, Up]),
            (1, -1) => self
                .get_chain_p_p(&[Right, Up])
                .min(self.get_chain_p_p(&[Up, Right])),
            (1, 1) => self
                .get_chain_p_p(&[Right, Down])
                .min(self.get_chain_p_p(&[Down, Right])),
            (-1, 1) if from == Up => self.get_chain_p_p(&[Down, Left]),
            (-1, 1) => self
                .get_chain_p_p(&[Left, Down])
                .min(self.get_chain_p_p(&[Down, Left])),
            (-1, -1) => self
                .get_chain_p_p(&[Left, Up])
                .min(self.get_chain_p_p(&[Up, Left])),
            (2, 0) => self.get_chain_p_p(&[Right, Right]),
            (-2, 0) => self.get_chain_p_p(&[Left, Left]),
            (2, -1) => self
                .get_chain_p_p(&[Right, Right, Up])
                .min(self.get_chain_p_p(&[Right, Up, Right])),
            (-2, 1) => self
                .get_chain_p_p(&[Left, Down, Left])
                .min(self.get_chain_p_p(&[Down, Left, Left])),
            _ => unreachable!(),
        }
    }

    fn shortest_numerical(&self, keypads: &Keypads, from: Numerical, to: Numerical) -> usize {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct State {
            pos: Vec2,
            prev: Directional,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        struct StateWithF(usize, State);

        impl PartialOrd for StateWithF {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for StateWithF {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.0.cmp(&other.0).reverse().then(self.1.cmp(&other.1))
            }
        }

        if from == to {
            return self.get(Directional::Press, Directional::Press);
        }

        let from = keypads.find_numerical_pos(from);
        let to = keypads.find_numerical_pos(to);
        let mut open = BinaryHeap::new();
        let init = State {
            pos: from,
            prev: Directional::Press,
        };
        open.push(StateWithF(0, init));
        let mut g_map = HashMap::new();
        g_map.insert(init, 0);
        let mut final_cost = usize::MAX;
        while let Some(StateWithF(_, q)) = open.pop() {
            let Some(g) = g_map.get(&q).copied() else {
                unreachable!();
            };
            for d in [
                Directional::Up,
                Directional::Down,
                Directional::Left,
                Directional::Right,
            ] {
                let new_pos = q.pos + d.as_vec2();
                let Some(Some(_)) = keypads.numerical.get(new_pos) else {
                    continue;
                };
                let new_g = g + self.get(q.prev, d);
                let new_q = State {
                    pos: new_pos,
                    prev: d,
                };
                if new_q.pos == to {
                    let total_cost = new_g + self.get(d, Directional::Press);
                    if final_cost > total_cost {
                        final_cost = total_cost;
                    }
                }
                let old_g = g_map.entry(new_q).or_insert(usize::MAX);
                if *old_g > new_g {
                    *old_g = new_g;
                    let h = 0;
                    let f = new_g + h;
                    open.push(StateWithF(f, new_q));
                }
            }
        }
        final_cost
    }
}

#[cfg(test)]
const INPUT: &str = "029A
980A
179A
456A
379A";

#[test]
fn cost_map_numerical() {
    let keypads = Keypads::new();
    let base = CostMap::base();
    assert_eq!(
        base.shortest_numerical(&keypads, Numerical::Press, Numerical::Press),
        1
    );
    assert_eq!(
        base.shortest_numerical(&keypads, Numerical::Press, Numerical::One),
        4
    );
}

#[test]
fn practice_part1() {
    assert_eq!(solve_part1(INPUT), 126384);
}

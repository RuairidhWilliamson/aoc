use std::{collections::HashMap, str::FromStr};

pub fn solve_part1(input: &str) -> usize {
    let (ordering_str, update_str) = input.split_once("\n\n").unwrap();
    let ordering: PrintOrdering = ordering_str.parse().unwrap();
    let updates: Vec<PrintUpdate> = update_str.lines().map(|l| l.parse().unwrap()).collect();
    updates
        .iter()
        .filter(|u| u.is_order_correct(&ordering))
        .map(|u| u.get_middle())
        .sum()
}

pub fn solve_part2(input: &str) -> usize {
    let (ordering_str, update_str) = input.split_once("\n\n").unwrap();
    let ordering: PrintOrdering = ordering_str.parse().unwrap();
    let updates: Vec<PrintUpdate> = update_str.lines().map(|l| l.parse().unwrap()).collect();
    updates
        .into_iter()
        .filter(|u| !u.is_order_correct(&ordering))
        .map(|mut u| {
            u.fix_order(&ordering);
            u
        })
        .map(|u| u.get_middle())
        .sum()
}

struct PrintOrdering {
    order: HashMap<usize, Vec<usize>>,
}

impl FromStr for PrintOrdering {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut order: HashMap<usize, Vec<usize>> = HashMap::default();
        for l in s.lines() {
            let (l, r) = l.split_once('|').unwrap();
            let (l, r) = (l.parse().unwrap(), r.parse().unwrap());
            order.entry(l).or_default().push(r);
        }
        Ok(Self { order })
    }
}

struct PrintUpdate {
    list: Vec<usize>,
}

impl FromStr for PrintUpdate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let list: Vec<usize> = s.split(',').map(|s| s.parse().unwrap()).collect();
        Ok(Self { list })
    }
}

impl PrintUpdate {
    fn is_order_correct(&self, ordering: &PrintOrdering) -> bool {
        for i in 0..self.list.len() {
            for j in i..self.list.len() {
                if ordering
                    .order
                    .get(&self.list[j])
                    .map_or(false, |v| v.contains(&self.list[i]))
                {
                    return false;
                }
            }
        }
        true
    }

    fn fix_order(&mut self, ordering: &PrintOrdering) {
        for i in 0..self.list.len() {
            for j in i..self.list.len() {
                if ordering
                    .order
                    .get(&self.list[j])
                    .map_or(false, |v| v.contains(&self.list[i]))
                {
                    self.list.swap(i, j);
                }
            }
        }
        debug_assert!(self.is_order_correct(ordering));
    }

    fn get_middle(&self) -> usize {
        *self.list.get(self.list.len() / 2).unwrap()
    }
}

#[cfg(test)]
const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

#[test]
fn practice_part1() {
    assert_eq!(solve_part1(INPUT), 143);
}

#[test]
fn practice_part2() {
    assert_eq!(solve_part2(INPUT), 123);
}

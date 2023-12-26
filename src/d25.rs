use std::collections::{HashMap, HashSet};

use crate::PartFn;

pub const PARTS: (PartFn, PartFn) = (part1, part2);

fn part1(input: &str) -> usize {
    let mut graph = parse_components(input);
    let [c1, c2] = graph.simple_min_cut().unwrap();
    c1 * c2
}

fn part2(_input: &str) -> usize {
    0
}

#[derive(Debug, Clone)]
pub struct Graph {
    vertices: Vec<(String, usize)>,
    edges: Vec<(usize, usize, usize)>,
}

pub fn parse_components(input: &str) -> Graph {
    let mut vertices = Vec::new();
    let mut edges = Vec::new();
    input.trim().trim_matches('\n').lines().for_each(|line| {
        let (left, right) = line.split_once(':').unwrap();
        let a = get_vertex_index_or_insert(&mut vertices, left);
        right.trim().split(' ').for_each(|x| {
            let b = get_vertex_index_or_insert(&mut vertices, x);
            edges.push((a, b, 1));
        })
    });

    Graph { vertices, edges }
}

fn get_vertex_index_or_insert(verts: &mut Vec<(String, usize)>, x: &str) -> usize {
    if let Some((i, _)) = verts.iter().enumerate().find(|&(_, (p, _))| p == x) {
        return i;
    }
    verts.push((x.to_owned(), 1));
    verts.len() - 1
}

impl Graph {
    #[allow(unused)]
    fn get_vertex_id(&self, v: &str) -> Option<usize> {
        self.vertices
            .iter()
            .enumerate()
            .find(|&(_, (p, _))| p == v)
            .map(|(i, _)| i)
    }

    fn find_adjacent(&self, i: usize) -> impl Iterator<Item = usize> + '_ {
        self.edges.iter().filter_map(move |(a, b, _)| {
            if *a == i {
                Some(*b)
            } else if *b == i {
                Some(*a)
            } else {
                None
            }
        })
    }

    fn find_adjacent_with_weight(&self, i: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.edges.iter().filter_map(move |(a, b, w)| {
            if *a == i {
                Some((*b, *w))
            } else if *b == i {
                Some((*a, *w))
            } else {
                None
            }
        })
    }

    fn find_adjacent_without<'a>(
        &'a self,
        i: usize,
        without: &'a [(usize, usize, usize)],
    ) -> impl Iterator<Item = usize> + 'a {
        self.edges.iter().filter_map(move |(a, b, _)| {
            if without.contains(&(*a, *b, 1)) || without.contains(&(*b, *a, 1)) {
                return None;
            }
            if *a == i {
                Some(*b)
            } else if *b == i {
                Some(*a)
            } else {
                None
            }
        })
    }

    fn find_connected_vert_count(&self, v: &str) -> Option<usize> {
        let mut to_visit = vec![self.get_vertex_id(v)?];
        let mut visited = HashSet::new();
        while let Some(i) = to_visit.pop() {
            if !visited.insert(i) {
                continue;
            }
            for i in self.find_adjacent(i) {
                to_visit.push(i);
            }
        }
        Some(visited.len() - 1)
    }

    fn find_connected_vert_count_without(
        &self,
        v: usize,
        without: &[(usize, usize, usize)],
    ) -> usize {
        let mut to_visit = vec![v];
        let mut visited = HashSet::new();
        while let Some(i) = to_visit.pop() {
            if !visited.insert(i) {
                continue;
            }
            for i in self.find_adjacent_without(i, without) {
                to_visit.push(i);
            }
        }
        visited.len() - 1
    }

    fn find_3_edges_to_split_in_2(&self) -> Option<[(usize, usize, usize); 3]> {
        for i in kdam::tqdm!(0..self.edges.len()) {
            for j in 0..i {
                for k in 0..j {
                    // println!("{i} {j} {k}");
                    let without = [self.edges[i], self.edges[j], self.edges[k]];
                    if self.split_in_2_count_groups(&without).is_some() {
                        return Some(without);
                    }
                }
            }
        }
        None
    }

    fn split_in_2_count_groups(&self, without: &[(usize, usize, usize)]) -> Option<[usize; 2]> {
        let mut to_visit = vec![0];
        let mut visited = HashSet::new();
        while let Some(i) = to_visit.pop() {
            if !visited.insert(i) {
                continue;
            }
            for i in self.find_adjacent_without(i, without) {
                to_visit.push(i);
            }
        }
        let not_visited: Vec<_> = (0..self.vertices.len())
            .filter(|i| !visited.contains(i))
            .collect();
        let i = not_visited.first()?;
        let other_connected_count = self.find_connected_vert_count_without(*i, without);
        if other_connected_count + 1 == not_visited.len() {
            Some([visited.len(), other_connected_count + 1])
        } else {
            None
        }
    }

    // Stoer–Wagner
    pub fn simple_min_cut(&mut self) -> Option<[usize; 2]> {
        let mut min_counts = None;
        let mut min_weight = None;
        let mut a = Vec::new();
        let mut h = HashMap::new();
        loop {
            a.clear();
            let mut start = 0;
            while let Some((_, count)) = self.vertices.get(start) {
                if *count != 0 {
                    break;
                }
                start += 1;
            }
            self.build_a_list(&mut a, start, &mut h);
            if a.len() < 2 {
                break;
            }
            let last = a.pop().unwrap();
            let weight: usize = self.find_adjacent_with_weight(last).map(|(_, w)| w).sum();
            let a_vert_count: usize = a.iter().map(|i| self.vertices[*i].1).sum();
            let last_vert_count = self.vertices[last].1;
            if min_weight.unwrap_or(usize::MAX) > weight {
                // println!("new min weight {weight}");
                min_counts = Some([a_vert_count, last_vert_count]);
                min_weight = Some(weight);
            }
            let second_last = a.pop().unwrap();
            self.merge_vertices(last, second_last);
        }
        // dbg!(min_weight);
        min_counts
    }

    fn build_a_list(&self, a: &mut Vec<usize>, start: usize, h: &mut HashMap<usize, usize>) {
        h.clear();
        a.push(start);
        let mut new_v = start;
        loop {
            for (x, y, w) in &self.edges {
                let v;
                if x == &new_v {
                    v = y;
                } else if y == &new_v {
                    v = x;
                } else {
                    continue;
                }
                if a.contains(v) {
                    continue;
                }
                *h.entry(*v).or_default() += w;
            }
            let Some(i) = h.iter().max_by_key(|(_, w)| *w).map(|(i, _)| *i) else {
                return;
            };
            h.remove(&i);
            a.push(i);
            new_v = i;
        }
    }

    fn find_max_adjacent(&self, a: &[usize]) -> Option<usize> {
        let mut h = HashMap::<usize, usize>::new();
        for (x, y, w) in &self.edges {
            let v;
            if a.contains(x) {
                v = y;
            } else if a.contains(y) {
                v = x;
            } else {
                continue;
            }
            if a.contains(v) {
                continue;
            }
            *h.entry(*v).or_default() += w;
        }
        h.into_iter().max_by_key(|(_, w)| *w).map(|(i, _)| i)
    }

    fn merge_vertices(&mut self, a: usize, b: usize) {
        let (a_name, a_count_ptr) = self.vertices.get_mut(a).unwrap();
        let a_count = *a_count_ptr;
        *a_count_ptr = 0;
        let a_name = a_name.to_owned();

        let (b_name, b_count_ptr) = self.vertices.get_mut(b).unwrap();
        let b_count = *b_count_ptr;
        *b_count_ptr = 0;
        let b_name = b_name.to_owned();

        let c = self.vertices.len();
        let new_vert = (format!("{a_name},{b_name}"), a_count + b_count);
        // println!("new vert {new_vert:?}");
        self.vertices.push(new_vert);

        let mut new_edges = HashMap::<usize, usize>::default();
        self.edges.retain(|&(x, y, w)| {
            if x == a && y == b || x == b && y == a {
                return false;
            }
            if x == a || x == b {
                *new_edges.entry(y).or_default() += w;
                false
            } else if y == a || y == b {
                *new_edges.entry(x).or_default() += w;
                false
            } else {
                true
            }
        });
        for (x, w) in new_edges {
            self.edges.push((c, x, w));
        }
    }
}

#[test]
fn example1() {
    let input = "
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
    ";

    let mut graph = parse_components(input);
    let [c1, c2] = graph.simple_min_cut().unwrap();
    assert_eq!(c1 * c2, 54);
}

#[test]
fn example2() {
    let input = "
A: B
B: C D
C: D
    ";

    let mut graph = parse_components(input);
    let [c1, c2] = graph.simple_min_cut().unwrap();
    assert_eq!(c1 * c2, 3);
}

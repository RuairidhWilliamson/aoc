pub struct MagicIterVec {
    state: Vec<usize>,
    first: bool,
    monotonic_sum: usize,
}

impl MagicIterVec {
    pub fn new(size: usize) -> Self {
        Self {
            state: vec![0; size],
            first: true,
            monotonic_sum: 0,
        }
    }

    pub fn state(&self) -> &Vec<usize> {
        &self.state
    }

    pub fn monotonic_sum(&self) -> usize {
        self.monotonic_sum
    }

    pub fn update_next(&mut self) -> bool {
        next_implementation(&mut self.first, &mut self.state, &mut self.monotonic_sum).is_some()
    }

    pub fn next_borrow(&mut self) -> Option<&Vec<usize>> {
        next_implementation(&mut self.first, &mut self.state, &mut self.monotonic_sum)?;
        Some(&self.state)
    }
}

impl Iterator for MagicIterVec {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        next_implementation(&mut self.first, &mut self.state, &mut self.monotonic_sum)?;
        Some(self.state.clone())
    }
}

pub struct MagicIterArr<const N: usize> {
    state: [usize; N],
    first: bool,
    monotonic_sum: usize,
}

impl<const N: usize> MagicIterArr<N> {
    pub fn new() -> Self {
        Self {
            first: true,
            state: [0; N],
            monotonic_sum: 0,
        }
    }

    pub fn state(&self) -> &[usize; N] {
        &self.state
    }

    pub fn monotonic_sum(&self) -> usize {
        self.monotonic_sum
    }

    pub fn update_next(&mut self) -> bool {
        next_implementation(&mut self.first, &mut self.state, &mut self.monotonic_sum).is_some()
    }

    pub fn next_borrow(&mut self) -> Option<&[usize; N]> {
        next_implementation(&mut self.first, &mut self.state, &mut self.monotonic_sum)?;
        Some(&self.state)
    }
}

impl<const N: usize> Iterator for MagicIterArr<N> {
    type Item = [usize; N];

    fn next(&mut self) -> Option<Self::Item> {
        next_implementation(&mut self.first, &mut self.state, &mut self.monotonic_sum)?;
        Some(self.state)
    }
}

#[must_use]
fn next_implementation(
    first: &mut bool,
    state: &mut [usize],
    monotonic_sum: &mut usize,
) -> Option<()> {
    if *first {
        *first = false;
        return Some(());
    }
    if state.len() == 0 {
        return None;
    }
    if do_the_shuffle(state) {
        return Some(());
    }
    *monotonic_sum += 1;
    state[1..].fill(0);
    state[0] = *monotonic_sum;
    Some(())
}

fn do_the_shuffle(state: &mut [usize]) -> bool {
    let width = state.len();
    if width <= 1 {
        return false;
    }
    if do_the_shuffle(&mut state[1..]) {
        return true;
    }
    if state[0] == 0 {
        return false;
    }
    let sum: usize = state[1..].iter().sum();
    state[0] -= 1;
    state[1..].fill(0);
    state[1] = sum + 1;
    true
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use itertools::{Itertools as _, repeat_n};

    use super::{MagicIterArr, MagicIterVec};

    #[test]
    fn test_magic_iter_arr_zero() {
        const N: usize = 0;
        let mut magic_iter = MagicIterVec::new(N);
        assert_eq!(magic_iter.next().unwrap(), vec![]);
        assert_eq!(magic_iter.next(), None);
    }

    #[test]
    fn test_magic_iter_arr_one() {
        const N: usize = 1;
        let mut magic_iter = MagicIterArr::<N>::new();
        for i in 0..7 {
            test_magic_iter_arr::<N>(&mut magic_iter, i);
        }
    }

    #[test]
    fn test_magic_iter_arr_two() {
        const N: usize = 2;
        let mut magic_iter = MagicIterArr::<N>::new();
        for i in 0..7 {
            test_magic_iter_arr::<N>(&mut magic_iter, i);
        }
    }

    #[test]
    fn test_magic_iter_arr_three() {
        const N: usize = 3;
        let mut magic_iter = MagicIterArr::<N>::new();
        for i in 0..7 {
            test_magic_iter_arr::<N>(&mut magic_iter, i);
        }
    }

    #[test]
    fn test_magic_iter_arr_four() {
        const N: usize = 4;
        let mut magic_iter = MagicIterArr::<N>::new();
        for i in 0..7 {
            test_magic_iter_arr::<N>(&mut magic_iter, i);
        }
    }

    #[cfg(test)]
    fn test_magic_iter_arr<const N: usize>(mut iter: impl Iterator<Item = [usize; N]>, sum: usize) {
        let expected: HashSet<Vec<usize>> = repeat_n(0..=sum, N)
            .multi_cartesian_product()
            .filter(|s| s.iter().sum::<usize>() == sum)
            .collect();
        let mut actual = HashSet::new();
        while actual.len() < expected.len() {
            actual.insert(iter.next().unwrap().to_vec());
        }
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_magic_iter_vec_zero() {
        const N: usize = 0;
        let mut magic_iter = MagicIterVec::new(N);
        assert_eq!(magic_iter.next().unwrap(), vec![]);
        assert_eq!(magic_iter.next(), None);
    }

    #[test]
    fn test_magic_iter_vec_one() {
        const N: usize = 1;
        let mut magic_iter = MagicIterVec::new(N);
        for i in 0..7 {
            test_magic_iter_vec::<N>(&mut magic_iter, i);
        }
    }

    #[test]
    fn test_magic_iter_vec_two() {
        const N: usize = 2;
        let mut magic_iter = MagicIterVec::new(N);
        for i in 0..7 {
            test_magic_iter_vec::<N>(&mut magic_iter, i);
        }
    }

    #[test]
    fn test_magic_iter_vec_three() {
        const N: usize = 3;
        let mut magic_iter = MagicIterVec::new(N);
        for i in 0..7 {
            test_magic_iter_vec::<N>(&mut magic_iter, i);
        }
    }

    #[test]
    fn test_magic_iter_vec_four() {
        const N: usize = 4;
        let mut magic_iter = MagicIterVec::new(N);
        for i in 0..7 {
            test_magic_iter_vec::<N>(&mut magic_iter, i);
        }
    }

    fn test_magic_iter_vec<'a, 'b, const N: usize>(
        mut iter: impl Iterator<Item = Vec<usize>>,
        sum: usize,
    ) where
        'b: 'a,
    {
        let expected: HashSet<Vec<usize>> = repeat_n(0..=sum, N)
            .multi_cartesian_product()
            .filter(|s| s.iter().sum::<usize>() == sum)
            .collect();
        let mut actual = HashSet::new();
        while actual.len() < expected.len() {
            actual.insert(iter.next().unwrap());
        }
        assert_eq!(expected, actual);
    }
}

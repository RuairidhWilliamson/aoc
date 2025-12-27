use std::iter::FusedIterator;

/// [`IntegerIter`] is an iterator that iterates over the space of 0 or positive integers in N dimensions.
/// It guarantees that the sum of the vector is monotonic meaning it either increases or stays the step at each iteration. It never decreases.
/// The implementation of [`Iterator`] requires a copy so use the other methods if you want to avoid extra allocations/memcpy.
///
/// # Example
/// ```
/// use aoc::integer_iter::IntegerIter;
///
/// let mut iter = IntegerIter::new_vec(3);
/// assert_eq!(iter.next(), Some(vec![0, 0, 0]));
/// assert_eq!(iter.next(), Some(vec![1, 0, 0]));
/// assert_eq!(iter.next(), Some(vec![0, 1, 0]));
/// assert_eq!(iter.next(), Some(vec![0, 0, 1]));
/// assert_eq!(iter.next(), Some(vec![2, 0, 0]));
/// assert_eq!(iter.next(), Some(vec![1, 1, 0]));
/// ```
///
pub struct IntegerIter<T> {
    state: T,
    first: bool,
    monotonic_sum: usize,
}

#[inline]
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
    if state.is_empty() {
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

impl IntegerIter<Vec<usize>> {
    /// Create a new integer iterator of size `size` using the heap to store the internal state
    pub fn new_vec(size: usize) -> Self {
        Self {
            state: vec![0; size],
            first: true,
            monotonic_sum: 0,
        }
    }

    /// The current state of the iterator
    pub fn state(&self) -> &Vec<usize> {
        &self.state
    }

    /// The current sum of the state of the iterator, never decreases
    pub fn monotonic_sum(&self) -> usize {
        self.monotonic_sum
    }

    /// Updates the state to the next item, returning false if the iterator is finished
    pub fn update_next(&mut self) -> bool {
        next_implementation(&mut self.first, &mut self.state, &mut self.monotonic_sum).is_some()
    }

    /// Get the next state as a borrowed vec, this is equivalent to calling [`Self::update_next`] and [`Self::state`]
    pub fn next_borrow(&mut self) -> Option<&Vec<usize>> {
        next_implementation(&mut self.first, &mut self.state, &mut self.monotonic_sum)?;
        Some(&self.state)
    }
}

impl Iterator for IntegerIter<Vec<usize>> {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        next_implementation(&mut self.first, &mut self.state, &mut self.monotonic_sum)?;
        Some(self.state.clone())
    }
}

impl FusedIterator for IntegerIter<Vec<usize>> {}

impl<const N: usize> IntegerIter<[usize; N]> {
    /// Create a new integer iterator of size `size` using the stack to store the internal state
    pub fn new_arr() -> Self {
        Self {
            state: [0; N],
            first: true,
            monotonic_sum: 0,
        }
    }

    /// The current state of the iterator
    pub fn state(&self) -> &[usize; N] {
        &self.state
    }

    /// The current sum of the state of the iterator, never decreases
    pub fn monotonic_sum(&self) -> usize {
        self.monotonic_sum
    }

    /// Updates the state to the next item, returning false if the iterator is finished
    pub fn update_next(&mut self) -> bool {
        next_implementation(&mut self.first, &mut self.state, &mut self.monotonic_sum).is_some()
    }

    /// Get the next state as a borrowed vec, this is equivalent to calling [`Self::update_next`] and [`Self::state`]
    pub fn next_borrow(&mut self) -> Option<&[usize; N]> {
        next_implementation(&mut self.first, &mut self.state, &mut self.monotonic_sum)?;
        Some(&self.state)
    }
}

impl<const N: usize> Iterator for IntegerIter<[usize; N]> {
    type Item = [usize; N];

    fn next(&mut self) -> Option<Self::Item> {
        next_implementation(&mut self.first, &mut self.state, &mut self.monotonic_sum)?;
        Some(self.state)
    }
}

impl<const N: usize> FusedIterator for IntegerIter<[usize; N]> {}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use itertools::{Itertools as _, repeat_n};

    use super::IntegerIter;

    #[test]
    fn test_integer_iter_arr_zero() {
        const N: usize = 0;
        let mut integer_iter = IntegerIter::<[usize; N]>::new_arr();
        assert_eq!(integer_iter.next().unwrap(), []);
        assert_eq!(integer_iter.next(), None);
    }

    #[test]
    fn test_integer_iter_arr_one() {
        const N: usize = 1;
        let mut integer_iter = IntegerIter::<[usize; N]>::new_arr();
        for i in 0..7 {
            test_integer_iter_arr::<N>(&mut integer_iter, i);
        }
    }

    #[test]
    fn test_integer_iter_arr_two() {
        const N: usize = 2;
        let mut integer_iter = IntegerIter::<[usize; N]>::new_arr();
        for i in 0..7 {
            test_integer_iter_arr::<N>(&mut integer_iter, i);
        }
    }

    #[test]
    fn test_integer_iter_arr_three() {
        const N: usize = 3;
        let mut integer_iter = IntegerIter::<[usize; N]>::new_arr();
        for i in 0..7 {
            test_integer_iter_arr::<N>(&mut integer_iter, i);
        }
    }

    #[test]
    fn test_integer_iter_arr_four() {
        const N: usize = 4;
        let mut integer_iter = IntegerIter::<[usize; N]>::new_arr();
        for i in 0..7 {
            test_integer_iter_arr::<N>(&mut integer_iter, i);
        }
    }

    #[cfg(test)]
    fn test_integer_iter_arr<const N: usize>(
        mut iter: impl Iterator<Item = [usize; N]>,
        sum: usize,
    ) {
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
    fn test_integer_iter_vec_zero() {
        const N: usize = 0;
        let mut integer_iter = IntegerIter::new_vec(N);
        assert_eq!(integer_iter.next().unwrap(), vec![]);
        assert_eq!(integer_iter.next(), None);
    }

    #[test]
    fn test_integer_iter_vec_one() {
        const N: usize = 1;
        let mut integer_iter = IntegerIter::new_vec(N);
        for i in 0..7 {
            test_integer_iter_vec::<N>(&mut integer_iter, i);
        }
    }

    #[test]
    fn test_integer_iter_vec_two() {
        const N: usize = 2;
        let mut integer_iter = IntegerIter::new_vec(N);
        for i in 0..7 {
            test_integer_iter_vec::<N>(&mut integer_iter, i);
        }
    }

    #[test]
    fn test_integer_iter_vec_three() {
        const N: usize = 3;
        let mut integer_iter = IntegerIter::new_vec(N);
        for i in 0..7 {
            test_integer_iter_vec::<N>(&mut integer_iter, i);
        }
    }

    #[test]
    fn test_integer_iter_vec_four() {
        const N: usize = 4;
        let mut integer_iter = IntegerIter::new_vec(N);
        for i in 0..7 {
            test_integer_iter_vec::<N>(&mut integer_iter, i);
        }
    }

    fn test_integer_iter_vec<'a, 'b, const N: usize>(
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

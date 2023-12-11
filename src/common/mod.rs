pub mod grid;

pub fn all_eq<I, T>(iter: &mut I) -> bool
where
    I: Iterator<Item = T>,
    T: PartialEq,
{
    let Some(first) = iter.next() else {
        return true;
    };
    iter.all(|e| first == e)
}

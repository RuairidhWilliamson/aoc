use std::borrow::Cow;

struct AsciiGrid<'a> {
    contents: Cow<'a, str>,
    width: usize,
    height: usize,
}

impl<'a> AsciiGrid<'a> {
    fn new(contents: impl Into<Cow<'a, str>>) -> Self {
        let contents = contents.into();
        assert!(contents.is_ascii());
        let width = contents.find('\n').unwrap();
        let height = contents.len() / (width + 1);
        assert_eq!((width + 1) * height, contents.len());
        Self {
            contents,
            width,
            height,
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<u8> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(unsafe { self.get_unchecked(x, y) })
    }

    unsafe fn get_unchecked(&self, x: usize, y: usize) -> u8 {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);
        let index = x + y * (self.width + 1);
        debug_assert!(index < self.contents.len());
        unsafe { *self.contents.as_bytes().get_unchecked(index) }
    }

    unsafe fn set_unchecked(&mut self, x: usize, y: usize, c: u8) {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);
        let index = x + y * (self.width + 1);
        debug_assert!(index < self.contents.len());
        unsafe {
            *self.contents.to_mut().as_mut_vec().get_unchecked_mut(index) = c;
        }
    }

    fn adjacent(&self, x: usize, y: usize) -> impl Iterator<Item = u8> {
        [
            (x.checked_sub(1), y.checked_sub(1)),
            (x.checked_add(0), y.checked_sub(1)),
            (x.checked_add(1), y.checked_sub(1)),
            (x.checked_sub(1), y.checked_add(0)),
            (x.checked_add(1), y.checked_add(0)),
            (x.checked_sub(1), y.checked_add(1)),
            (x.checked_add(0), y.checked_add(1)),
            (x.checked_add(1), y.checked_add(1)),
        ]
        .into_iter()
        .filter_map(|(x, y)| self.get(x?, y?))
    }
}

fn is_accessible(grid: &AsciiGrid, x: usize, y: usize) -> bool {
    grid.adjacent(x, y).filter(|c| *c == b'@').count() < 4
}

pub fn part1(input: &str) -> u32 {
    let grid = AsciiGrid::new(input);
    let mut accessible_rolls = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            if unsafe { grid.get_unchecked(x, y) } != b'@' {
                continue;
            }
            if is_accessible(&grid, x, y) {
                accessible_rolls += 1;
            }
        }
    }
    accessible_rolls
}

pub fn part2(input: &str) -> usize {
    let mut grid = AsciiGrid::new(input);
    let mut total_removed_rolls = 0;
    let mut accessible_rolls = Vec::new();
    loop {
        for y in 0..grid.height {
            for x in 0..grid.width {
                if unsafe { grid.get_unchecked(x, y) } == b'.' {
                    continue;
                }
                if is_accessible(&grid, x, y) {
                    accessible_rolls.push((x, y));
                }
            }
        }
        if accessible_rolls.is_empty() {
            break;
        }
        for (x, y) in &accessible_rolls {
            unsafe { grid.set_unchecked(*x, *y, b'.') };
        }
        total_removed_rolls += accessible_rolls.len();
        accessible_rolls.clear();
    }
    total_removed_rolls
}

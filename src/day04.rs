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

    fn get(&self, x: usize, y: usize) -> Option<char> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let index = x + y * (self.width + 1);
        Some(self.contents.as_bytes()[index] as char)
    }

    fn set(&mut self, x: usize, y: usize, c: char) {
        assert!(x < self.width);
        assert!(y < self.height);
        let index = x + y * (self.width + 1);
        assert!(c.is_ascii());
        unsafe {
            self.contents.to_mut().as_mut_vec()[index] = c as u8;
        }
    }

    fn get_signed(&self, x: isize, y: isize) -> Option<char> {
        if x < 0 || y < 0 {
            return None;
        }
        self.get(x as usize, y as usize)
    }
}

pub fn part1(input: &str) -> u32 {
    let grid = AsciiGrid::new(input);
    let mut accessible_rolls = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.get(x, y) != Some('@') {
                continue;
            }
            if is_accessible(&grid, x, y) {
                accessible_rolls += 1;
            }
        }
    }
    accessible_rolls
}

fn is_accessible(grid: &AsciiGrid, x: usize, y: usize) -> bool {
    let mut roll_count = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if !(i == 0 && j == 0)
                && let Some(c) = grid.get_signed(x as isize + i, y as isize + j)
                && c == '@'
            {
                roll_count += 1;
            }
        }
    }
    roll_count < 4
}

pub fn part2(input: &str) -> u32 {
    let mut grid = AsciiGrid::new(input);
    let mut removed_rolls = 0;
    loop {
        let mut accessible_rolls = Vec::new();
        for y in 0..grid.height {
            for x in 0..grid.width {
                if grid.get(x, y) != Some('@') {
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
        for (x, y) in accessible_rolls {
            grid.set(x, y, '.');
            removed_rolls += 1;
        }
    }
    removed_rolls
}

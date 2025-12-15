use crate::ascii_grid::AsciiGrid;

fn is_accessible(grid: &AsciiGrid, x: usize, y: usize) -> bool {
    grid.adjacent(x, y).filter(|c| *c == b'@').count() < 4
}

pub fn part1(input: &str) -> u32 {
    let grid = AsciiGrid::new(input.as_bytes());
    let mut accessible_rolls = 0;
    for y in 0..grid.height() {
        for x in 0..grid.width() {
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
    let mut grid = AsciiGrid::new(input.as_bytes());
    let mut total_removed_rolls = 0;
    let mut accessible_rolls = Vec::new();
    loop {
        for y in 0..grid.height() {
            for x in 0..grid.width() {
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

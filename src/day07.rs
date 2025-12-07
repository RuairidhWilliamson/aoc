use crate::grid::ByteGrid;

pub fn part1(input: &str) -> usize {
    let mut grid = ByteGrid::new(input.as_bytes());
    let mut split_count = 0;
    for y in 0..grid.height() - 2 {
        for x in 0..grid.width() {
            let c = unsafe { grid.get_unchecked(x, y) };
            match c {
                b'S' | b'|' => {
                    if unsafe { grid.get_unchecked(x, y + 1) } == b'^' {
                        if x > 0 {
                            unsafe { grid.set_unchecked(x - 1, y + 1, b'|') };
                        }
                        if x + 1 < grid.width() {
                            unsafe { grid.set_unchecked(x + 1, y + 1, b'|') };
                        }
                        split_count += 1;
                    } else {
                        unsafe { grid.set_unchecked(x, y + 1, b'|') };
                    }
                }
                b'.' | b'^' => {}
                c => {
                    panic!("unexpected symbol {c}")
                }
            }
        }
    }
    split_count
}

pub fn part2(input: &str) -> usize {
    let grid = ByteGrid::new(input.as_bytes());
    let mut counts = vec![0usize; grid.width()];
    let mut new_counts = vec![0usize; grid.width()];
    let start_x = (0..grid.width())
        .find(|x| {
            let c = unsafe { grid.get_unchecked(*x, 0) };
            c == b'S'
        })
        .unwrap();
    counts[start_x] = 1;
    for y in 1..grid.height() {
        for x in 0..grid.width() {
            let c = unsafe { grid.get_unchecked(x, y) };
            match c {
                b'.' => {
                    new_counts[x] += counts[x];
                }
                b'^' => {
                    new_counts.get_mut(x - 1).map(|v| *v += counts[x]);
                    new_counts.get_mut(x + 1).map(|v| *v += counts[x]);
                }
                c => panic!("unexpected symbol {c}"),
            }
        }
        std::mem::swap(&mut counts, &mut new_counts);
        for x in &mut new_counts {
            *x = 0;
        }
    }
    counts.iter().sum()
}

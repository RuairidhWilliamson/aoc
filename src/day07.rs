use crate::grid::ByteGrid;

pub fn part1(input: &str) -> usize {
    let mut grid = ByteGrid::new(input.as_bytes());
    let mut split_count = 0;
    for y in 0..grid.height() - 1 {
        for x in 0..grid.width() {
            let c = grid.get(x, y).unwrap();
            match c {
                b'S' | b'|' => {
                    if grid.get(x, y + 1).unwrap() == b'^' {
                        let _ = grid.set(x - 1, y + 1, b'|');
                        let _ = grid.set(x + 1, y + 1, b'|');
                        split_count += 1;
                    } else {
                        grid.set(x, y + 1, b'|').unwrap();
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
    let start_x = (0..grid.width())
        .find(|x| {
            let c = grid.get(*x, 0).unwrap();
            c == b'S'
        })
        .unwrap();
    let mut counts = vec![0usize; grid.width()];
    let mut new_counts = counts.clone();
    counts[start_x] = 1;
    for y in 1..grid.height() {
        for x in 0..grid.width() {
            let c = grid.get(x, y).unwrap();
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
        new_counts.fill(0);
    }
    counts.iter().sum()
}

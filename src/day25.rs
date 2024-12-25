#[expect(clippy::needless_range_loop)]
pub fn solve_part1(input: &str) -> usize {
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    input.split("\n\n").for_each(|spec| {
        let mut lines = spec.lines();
        let first = lines.next().unwrap();
        let mut heights = [0u8; 5];
        match first {
            "#####" => {
                for l in lines {
                    for i in 0..l.len() {
                        if l.chars().nth(i).unwrap() == '#' {
                            heights[i] += 1;
                        }
                    }
                }
                locks.push(heights);
            }
            "....." => {
                let _ = lines.next_back().unwrap();
                for l in lines.rev() {
                    for i in 0..l.len() {
                        if l.chars().nth(i).unwrap() == '#' {
                            heights[i] += 1;
                        }
                    }
                }
                keys.push(heights);
            }
            _ => panic!("invalid lock/key"),
        }
    });
    locks
        .into_iter()
        .map(|lock| {
            keys.iter()
                .filter(|key| {
                    let res = key.iter().zip(lock.iter()).all(|(k, l)| k + l <= 5);
                    res
                })
                .count()
        })
        .sum()
}

pub fn solve_part2(_input: &str) -> usize {
    0
}

#[cfg(test)]
const INPUT: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

#[test]
fn practice_part1() {
    assert_eq!(solve_part1(INPUT), 3);
}

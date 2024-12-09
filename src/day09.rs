use std::{convert::Infallible, num::NonZeroUsize, str::FromStr};

pub fn solve_part1(input: &str) -> usize {
    let mut disk: Disk = input.trim().parse().unwrap();
    disk.compact();
    disk.checksum()
}

pub fn solve_part2(input: &str) -> usize {
    let mut disk: Disk = input.trim().parse().unwrap();
    disk.compact2();
    disk.checksum()
}

#[derive(Debug)]
struct Disk {
    blocks: Vec<Block>,
}

impl FromStr for Disk {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut free = false;
        let mut file_id = 0;
        let blocks: Vec<Block> = s
            .chars()
            .map(|size| {
                assert!(size.is_ascii_digit());
                let size: usize = size as usize - '0' as usize;
                let b = Block {
                    file_id: if free { None } else { Some(file_id) },
                    size,
                };
                if !free {
                    file_id += 1;
                }
                free = !free;
                b
            })
            .collect();
        Ok(Disk { blocks })
    }
}

impl Disk {
    fn compact(&mut self) {
        loop {
            self.remove_trailing_free_blocks();
            let Some((free_block_index, _)) =
                self.blocks.iter().enumerate().find(|(_, b)| b.is_free())
            else {
                return;
            };
            let Some(mut last_block) = self.blocks.pop() else {
                unreachable!()
            };
            debug_assert!(last_block.is_file());
            let free_block = &mut self.blocks[free_block_index];
            match last_block.size.cmp(&free_block.size) {
                std::cmp::Ordering::Less => {
                    free_block.size -= last_block.size;
                    self.blocks.insert(free_block_index, last_block);
                }
                std::cmp::Ordering::Greater | std::cmp::Ordering::Equal => {
                    free_block.file_id = last_block.file_id;
                    last_block.size -= free_block.size;
                    if last_block.size > 0 {
                        self.blocks.push(last_block);
                    }
                }
            }
        }
    }

    fn compact2(&mut self) {
        let mut largest_attempted_file_id = usize::MAX;
        loop {
            let Some((file_block_index, file_block)) = self
                .blocks
                .iter()
                .enumerate()
                .rfind(|(_, b)| b.file_id.is_some_and(|id| id < largest_attempted_file_id))
            else {
                return;
            };
            largest_attempted_file_id = file_block.file_id.unwrap();
            let Some((free_block_index, _)) = self
                .blocks
                .iter()
                .enumerate()
                .take(file_block_index)
                .find(|(_, b)| b.is_free() && b.size >= file_block.size)
            else {
                continue;
            };
            let file_block = Block {
                file_id: file_block.file_id,
                size: file_block.size,
            };
            self.blocks[file_block_index].file_id = None;
            let free_block = &mut self.blocks[free_block_index];
            if free_block.size == file_block.size {
                free_block.file_id = file_block.file_id;
            } else {
                free_block.size -= file_block.size;
                self.blocks.insert(free_block_index, file_block);
            }
        }
    }

    fn remove_trailing_free_blocks(&mut self) {
        for i in (0..self.blocks.len()).rev() {
            if !self.blocks[i].is_free() {
                break;
            }
            self.blocks.remove(i);
        }
    }

    fn checksum(&self) -> usize {
        let mut position = 0;
        self.blocks
            .iter()
            .map(|b| {
                let out = b.checksum(position);
                position += b.size;
                out
            })
            .sum()
    }
}

impl std::fmt::Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in &self.blocks {
            for _ in 0..b.size {
                if let Some(id) = b.file_id {
                    f.write_fmt(format_args!("{id}"))?;
                } else {
                    f.write_str(".")?;
                }
            }
        }
        Ok(())
    }
}

fn triangle(n: usize) -> usize {
    (n + 1) * n / 2
}

#[derive(Debug)]
struct Block {
    file_id: Option<usize>,
    size: usize,
}

impl Block {
    fn is_free(&self) -> bool {
        self.file_id.is_none()
    }

    fn is_file(&self) -> bool {
        self.file_id.is_some()
    }

    fn checksum(&self, position: usize) -> usize {
        let Some(file_id) = self.file_id else {
            return 0;
        };
        if let Some(pos) = NonZeroUsize::new(position) {
            file_id * (triangle(self.size + pos.get() - 1) - triangle(pos.get() - 1))
        } else {
            file_id * triangle(self.size - 1)
        }
    }
}

#[cfg(test)]
const INPUT: &str = "2333133121414131402";

#[test]
fn practice_part1() {
    assert_eq!(solve_part1(INPUT), 1928);
}

#[test]
fn block_checksum() {
    assert_eq!(
        Block {
            file_id: Some(1),
            size: 2
        }
        .checksum(0),
        1
    );
}

#[test]
fn practice_part2() {
    assert_eq!(solve_part2(INPUT), 2858);
}

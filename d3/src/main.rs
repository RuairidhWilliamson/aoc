use std::{io::stdin, str::FromStr};

fn main() {
    let input = std::io::read_to_string(stdin().lock()).unwrap();
    let total = find_total(&input);
    println!("Total = {total}");
    let total_ratios = find_total_gear_ratios(&input);
    println!("Total Gear Ratios = {total_ratios}");
}

fn find_parts(grid: &Grid<char>) -> Vec<Part> {
    let mut acc = Vec::default();
    for y in 0..grid.height() {
        let mut num_acc = None;
        for x in 0..grid.width() {
            let c = grid.get((x, y)).unwrap();
            if c.is_ascii_digit() {
                let mut part = num_acc.take().unwrap_or(PartNumberAcc {
                    acc: 0,
                    start: x,
                    end: x,
                    y,
                });
                part.acc = part.acc * 10 + *c as u64 - '0' as u64;
                part.end = x;
                num_acc = Some(part);
            } else if *c == '.' || c.is_ascii_punctuation() {
                if let Some(part) = num_acc.take() {
                    if let Some(coord) = part.find_symbol(&grid) {
                        acc.push(Part {
                            number: part.acc,
                            coord,
                            symbol: *grid.get(coord).unwrap(),
                        });
                    }
                }
            } else {
                panic!("{c}");
            }
        }
        if let Some(part) = num_acc.take() {
            if let Some(coord) = part.find_symbol(&grid) {
                acc.push(Part {
                    number: part.acc,
                    coord,
                    symbol: *grid.get(coord).unwrap(),
                });
            }
        }
    }
    acc
}

fn find_total(input: &str) -> u64 {
    let grid: Grid<char> = input.parse().unwrap();
    let parts = find_parts(&grid);
    parts.iter().map(|p| p.number).sum()
}

fn find_total_gear_ratios(input: &str) -> u64 {
    let grid: Grid<char> = input.parse().unwrap();
    let parts: Vec<Part> = find_parts(&grid)
        .into_iter()
        .filter(|p| p.symbol == '*')
        .collect();
    (0..parts.len())
        .filter_map(|i| {
            let p1 = &parts[i];
            let p2s: Vec<&Part> = parts[..i]
                .iter()
                .filter(|p2| p2.coord == p1.coord)
                .collect();
            if p2s.len() != 1 {
                return None;
            }
            let p2 = p2s[0];
            Some(p1.number * p2.number)
        })
        .sum()
}

struct PartNumberAcc {
    acc: u64,
    start: isize,
    end: isize,
    y: isize,
}

impl PartNumberAcc {
    fn find_symbol(&self, grid: &Grid<char>) -> Option<(isize, isize)> {
        iter_product(self.start - 1..=self.end + 1, self.y - 1..=self.y + 1).find_map(|coord| {
            grid.get(coord).and_then(|&c| {
                if c != '.' && c.is_ascii_punctuation() {
                    Some(coord)
                } else {
                    None
                }
            })
        })
    }
}

fn iter_product<T: Copy, U, I: Iterator<Item = U> + Clone>(
    iter1: impl Iterator<Item = T>,
    iter2: I,
) -> impl Iterator<Item = (T, U)> {
    iter1.flat_map(move |t| iter2.clone().map(move |u| (t, u)))
}

struct Part {
    number: u64,
    coord: (isize, isize),
    symbol: char,
}

struct Grid<T> {
    elements: Vec<T>,
    width: isize,
}

impl std::fmt::Display for Grid<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.height() {
            let index = (i * self.width) as usize..(i * self.width + self.width) as usize;
            f.write_str(&String::from_iter(&self.elements[index]))?;
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl<T> Grid<T> {
    fn calc_index(&self, coord: (isize, isize)) -> Option<isize> {
        if !(0..self.width()).contains(&coord.0) || !(0..self.height()).contains(&coord.1) {
            return None;
        }
        Some(self.width * coord.1 + coord.0)
    }

    pub fn get(&self, coord: (isize, isize)) -> Option<&T> {
        self.elements.get(self.calc_index(coord)? as usize)
    }

    pub fn width(&self) -> isize {
        self.width
    }

    pub fn height(&self) -> isize {
        self.elements.len() as isize / self.width
    }
}

impl FromStr for Grid<char> {
    type Err = MyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !all_eq(&mut s.lines().map(|l| l.len())) {
            return Err(MyError::LinesAreNotSameLength);
        }
        let width = s.lines().next().unwrap().len() as isize;
        let elements = s
            .lines()
            .flat_map(|l| l.as_bytes().into_iter().map(|&c| c as char))
            .collect();
        Ok(Self { elements, width })
    }
}

#[derive(Debug)]
enum MyError {
    LinesAreNotSameLength,
}

fn all_eq<I, T>(iter: &mut I) -> bool
where
    I: Iterator<Item = T>,
    T: PartialEq,
{
    let Some(first) = iter.next() else {
        return true;
    };
    iter.all(|e| first == e)
}

#[cfg(test)]
mod tests {
    use crate::{find_total, find_total_gear_ratios};

    fn clean(input: &str) -> String {
        input.chars().filter(|&c| c != ' ').collect()
    }

    #[test]
    fn my_valid_examples1() {
        let inputs = &[
            "
            #...
            .5..
            ....
            ",
            "
            .#..
            .5..
            ....
            ",
            "
            ..#.
            .5..
            ....
            ",
            "
            ....
            #5..
            ....
            ",
            "
            ....
            .5#.
            ....
            ",
            "
            ....
            .5..
            #...
            ",
            "
            ....
            .5..
            .#..
            ",
            "
            ....
            .5..
            ..#.
            ",
        ];
        for input in inputs {
            assert_eq!(find_total(clean(input).trim_matches('\n')), 5, "{}", input);
        }
    }

    #[test]
    fn my_valid_examples2() {
        let inputs = &["
            #...
            ...9
            ..*.
            "];
        for input in inputs {
            assert_eq!(find_total(clean(input).trim_matches('\n')), 9, "{}", input);
        }
    }

    #[test]
    fn my_invalid_examples1() {
        let inputs = &[
            "
            #...
            ...9
            ....
            ",
            "
            .#..
            ...9
            ....
            ",
            "
            ....
            #..9
            ....
            ",
            "
            ....
            .#.9
            ....
            ",
            "
            ....
            ...9
            #...
            ",
            "
            ....
            ...9
            .#..
            ",
            "
            ##..
            ##.9
            ##..
            ",
        ];
        for input in inputs {
            assert_eq!(find_total(clean(input).trim_matches('\n')), 0, "{}", input);
        }
    }

    #[test]
    fn given_example1() {
        let input = "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        ";
        let input_cleaned = clean(input);
        let input_cleaned = input_cleaned.trim_matches('\n');
        assert_eq!(find_total(input_cleaned), 4361);
        assert_eq!(find_total_gear_ratios(input_cleaned), 467835);
    }
}

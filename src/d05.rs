use std::{num::ParseIntError, ops::Range, str::FromStr};

pub fn run(input: &str) {
    run_inner(input).expect("d05 failed");
}

fn run_inner(input: &str) -> Result<(), MyError> {
    let mut lines = input.lines();

    let seeds = lines
        .next()
        .ok_or(MyError::MissingLine)?
        .strip_prefix("seeds: ")
        .ok_or(MyError::NotSeeds)?
        .trim()
        .split(' ')
        .map(|x| x.parse())
        .collect::<Result<Vec<usize>, ParseIntError>>()?;

    let rest: Vec<_> = lines.collect();
    let almanac: Almanac = rest.join("\n").parse()?;

    let min_location = seeds
        .chunks_exact(2)
        .map(|s| s[0]..s[0] + s[1])
        .flat_map(|seed| almanac.map_ranges(vec![seed]).into_iter().map(|r| r.start))
        .min();
    println!("{min_location:?}");

    Ok(())
}

#[derive(Debug)]
struct Almanac {
    maps: Vec<Map>,
}

impl Almanac {
    #[allow(dead_code)]
    fn map(&self, id: usize) -> usize {
        self.maps.iter().fold(id, |id, m| m.map(id))
    }

    fn map_ranges(&self, rngs: Vec<Range<usize>>) -> Vec<Range<usize>> {
        self.maps
            .iter()
            .fold(rngs, |id_rngs, m| m.map_ranges(id_rngs))
    }
}

impl FromStr for Almanac {
    type Err = MyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let maps = s
            .split("\n\n")
            .map(|m| m.parse())
            .collect::<Result<Vec<Map>, MyError>>()?;
        Ok(Self { maps })
    }
}

#[derive(Debug)]
struct Map {
    #[allow(dead_code)]
    from: String,
    #[allow(dead_code)]
    to: String,
    entries: Vec<MapEntry>,
}

impl Map {
    fn map(&self, src: usize) -> usize {
        self.entries
            .iter()
            .find_map(|e| {
                if e.src_rng().contains(&src) {
                    Some(src - e.src + e.dst)
                } else {
                    None
                }
            })
            .unwrap_or(src)
    }

    fn map_ranges(&self, mut rngs: Vec<Range<usize>>) -> Vec<Range<usize>> {
        let mut out = Vec::default();
        'outer: while let Some(rng) = rngs.pop() {
            for e in &self.entries {
                match e.src_overlap(&rng) {
                    Some(Overlap::EntirelyWithin) => {
                        out.push(e.map(rng));
                        continue 'outer;
                    }
                    Some(Overlap::StartIn) => {
                        out.push(e.map(rng.start..e.src_end()));
                        rngs.push(e.src_end()..rng.end);
                        continue 'outer;
                    }
                    Some(Overlap::EndIn) => {
                        out.push(e.map(e.src..rng.end));
                        rngs.push(rng.start..e.src);
                        continue 'outer;
                    }
                    Some(Overlap::MiddlePortion) => {
                        out.push(e.map(e.src_rng()));
                        rngs.push(rng.start..e.src);
                        rngs.push(e.src_end()..rng.end);
                        continue 'outer;
                    }
                    None => (),
                }
            }
            out.push(rng);
        }
        out
    }
}

impl FromStr for Map {
    type Err = MyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, rest) = s.trim().split_once(' ').ok_or(MyError::MissingSpace)?;
        let (from, to) = name
            .split_once("-to-")
            .ok_or(MyError::MissingMapNameSeparator)?;
        let rest = rest
            .strip_prefix("map:")
            .ok_or(MyError::MissingMapKeyword)?;
        let entries = rest
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.parse::<MapEntry>())
            .collect::<Result<Vec<MapEntry>, MyError>>()?;
        Ok(Self {
            from: from.to_owned(),
            to: to.to_owned(),
            entries,
        })
    }
}

#[derive(Debug)]
struct MapEntry {
    dst: usize,
    src: usize,
    range: usize,
}

impl MapEntry {
    fn src_end(&self) -> usize {
        self.src + self.range
    }

    fn src_rng(&self) -> Range<usize> {
        self.src..self.src_end()
    }

    fn map(&self, mut rng: Range<usize>) -> Range<usize> {
        assert!(matches!(
            self.src_overlap(&rng),
            Some(Overlap::EntirelyWithin)
        ));
        rng.end = self.dst + rng.end - self.src;
        rng.start = self.dst + rng.start - self.src;
        rng
    }

    fn src_overlap(&self, rng: &Range<usize>) -> Option<Overlap> {
        match (
            self.src_rng().contains(&rng.start),
            self.src_rng().contains(&(rng.end - 1)),
        ) {
            (true, true) => Some(Overlap::EntirelyWithin),
            (true, false) => Some(Overlap::StartIn),
            (false, true) => Some(Overlap::EndIn),
            (false, false) => {
                if (rng.start < self.src) == (rng.end <= self.src) {
                    None
                } else {
                    Some(Overlap::MiddlePortion)
                }
            }
        }
    }
}

#[derive(Debug)]
enum Overlap {
    /// The rng is entirely within the map entry
    EntirelyWithin,
    /// The start of the rng is within the map entry but the end is not
    StartIn,
    /// The end of the rng is within the map entry but the start is not
    EndIn,
    /// The map entry is within the rng
    MiddlePortion,
}

impl FromStr for MapEntry {
    type Err = MyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .trim()
            .split(' ')
            .map(|x| x.parse::<usize>())
            .collect::<Result<Vec<usize>, ParseIntError>>()?;
        if numbers.len() != 3 {
            return Err(MyError::Expected3Numbers);
        }
        Ok(Self {
            dst: numbers[0],
            src: numbers[1],
            range: numbers[2],
        })
    }
}

#[derive(Debug, thiserror::Error)]
enum MyError {
    #[error("parse int: {0}")]
    ParseInt(#[from] ParseIntError),
    #[error("expected 3 numbers in a map entry")]
    Expected3Numbers,
    #[error("no space")]
    MissingSpace,
    #[error("missing -to- in map name")]
    MissingMapNameSeparator,
    #[error("missing map keyword")]
    MissingMapKeyword,
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("not seeds")]
    NotSeeds,
    #[error("missing line")]
    MissingLine,
}

#[cfg(test)]
mod tests {
    #![allow(clippy::single_range_in_vec_init)]
    use std::ops::Range;

    use super::Almanac;

    #[test]
    fn test_82() {
        let input = "
seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
        "
        .trim();
        let a: Almanac = input.parse().unwrap();
        let r = vec![82..83];
        let [v]: [Range<usize>; 1] = a.maps[0].map_ranges(r).try_into().unwrap();
        assert_eq!(v.start, 84);
        assert_eq!(v.end, 85);

        let r = vec![50..60];
        let v = a.maps[1].map_ranges(r);
        assert_eq!(v.len(), 3);
        assert_eq!(v[0], 35..37);
        assert_eq!(v[1], 37..39);
        assert_eq!(v[2], 54..60);

        let r = vec![20..30];
        let v = a.maps[3].map_ranges(r);
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], 90..95);
        assert_eq!(v[1], 18..23);

        let r = vec![70..80];
        let v = a.maps[5].map_ranges(r);
        println!("{v:?}");
        assert_eq!(v.len(), 1);
        assert_eq!(v[0], 70..80);

        use rand::RngCore;
        use rand::SeedableRng;
        let mut std_rng = rand::rngs::StdRng::seed_from_u64(42);
        for _ in 0..200 {
            let u = std_rng.next_u32() as usize % 250;
            let v = u + std_rng.next_u32() as usize % 250;
            let rngs = a.map_ranges(vec![u..v]);
            for j in 0..u {
                let o = a.map(j);
                assert!(!rngs.iter().any(|r| r.contains(&o)));
            }
            for j in u..v {
                let o = a.map(j);
                assert!(rngs.iter().any(|r| r.contains(&o)));
            }
            for j in v..500 {
                let o = a.map(j);
                assert!(!rngs.iter().any(|r| r.contains(&o)));
            }
        }
    }
}

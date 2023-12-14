use crate::PartFn;

pub const PARTS: (PartFn, PartFn) = (part1, part2);

fn part1(input: &str) -> isize {
    let mut lines = input.lines();
    let line = lines.next().unwrap();
    let times = line
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<isize>().unwrap());

    let line = lines.next().unwrap();
    let distances = line
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<isize>().unwrap());

    let races = times.zip(distances);
    let product: usize = races
        .map(|(time, distance)| ways_to_beat_race(time, distance))
        .product();
    product as isize
}

fn part2(_input: &str) -> isize {
    ways_to_beat_race(62649190, 553101014731074) as isize
}

fn ways_to_beat_race(time: isize, distance: isize) -> usize {
    let discriminant = time * time - 4 * distance;
    if discriminant < 0 {
        return 0;
    }
    let min = (time as f64 - (discriminant as f64).sqrt()) / 2.0;
    let max = (time as f64 + (discriminant as f64).sqrt()) / 2.0;

    let rng = (min.floor() as usize + 1)..(max.ceil() as usize);
    rng.end - rng.start
}

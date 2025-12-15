use std::collections::HashMap;

type Network<'a> = HashMap<&'a str, Vec<&'a str>>;

fn parse_network(input: &str) -> Network<'_> {
    input
        .lines()
        .map(|line| {
            let (from, rest) = line.split_once(':').unwrap();
            let to: Vec<&str> = rest.trim().split(' ').collect();
            (from, to)
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    fn visit<'a>(network: &Network<'a>, v: &'a str) -> usize {
        if v == "out" {
            return 1;
        }
        network
            .get(v)
            .unwrap()
            .iter()
            .map(|x| visit(network, x))
            .sum()
    }
    let network = parse_network(input);
    visit(&network, "you")
}

pub fn part2(input: &str) -> usize {
    fn visit<'a>(
        network: &Network<'a>,
        cache: &mut HashMap<(&'a str, bool, bool), usize>,
        v: &'a str,
        mut seen_dac: bool,
        mut seen_fft: bool,
    ) -> usize {
        if v == "out" {
            if seen_dac && seen_fft {
                return 1;
            }
            return 0;
        }
        if let Some(count) = cache.get(&(v, seen_dac, seen_fft)) {
            return *count;
        }
        if v == "dac" {
            seen_dac = true;
        }
        if v == "fft" {
            seen_fft = true;
        }
        let count = network
            .get(v)
            .unwrap()
            .iter()
            .map(|x| visit(network, cache, x, seen_dac, seen_fft))
            .sum();
        cache.insert((v, seen_dac, seen_fft), count);
        count
    }
    let network = parse_network(input);
    let mut cache = HashMap::new();
    visit(&network, &mut cache, "svr", false, false)
}

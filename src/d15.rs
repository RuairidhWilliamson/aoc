use std::collections::HashMap;

use crate::PartFn;

pub const PARTS: (PartFn, PartFn) = (part1, part2);

fn part1(input: &str) -> isize {
    input
        .trim()
        .trim_matches('\n')
        .split(',')
        .map(|s| hash(s) as isize)
        .sum()
}

fn part2(input: &str) -> isize {
    let mut entries = Entries::default();
    input.trim().trim_matches('\n').split(',').for_each(|s| {
        let symbol_index = s.find(['=', '-']).unwrap();
        let label = &s[..symbol_index];
        let symbol = &s[symbol_index..symbol_index + 1];
        let value = &s[symbol_index + 1..];
        let box_index = hash(label);
        let entry_list = entries.entries.entry(box_index).or_default();
        match symbol {
            "-" => {
                entry_list.retain(|e| e.label != label);
            }
            "=" => {
                let value: usize = value.parse().unwrap();
                if let Some(entry) = entry_list.iter_mut().find(|e| e.label == label) {
                    entry.value = value;
                } else {
                    entry_list.push(Entry {
                        label: label.to_owned(),
                        value,
                    })
                }
            }
            _ => panic!("unknown symbol"),
        }
    });
    entries
        .entries
        .iter()
        .map(|(i, es)| {
            es.iter()
                .enumerate()
                .map(|(j, e)| (*i as usize + 1) * (j + 1) * e.value)
                .sum::<usize>()
        })
        .sum::<usize>() as isize
}

fn hash(input: &str) -> u8 {
    let mut v: usize = 0;
    input.chars().for_each(|c| {
        v = (17 * (v + c as usize)) % 256;
    });
    v as u8
}

#[derive(Debug, Default)]
struct Entries {
    entries: HashMap<u8, Vec<Entry>>,
}

#[derive(Debug)]
struct Entry {
    label: String,
    value: usize,
}

#[test]
fn hash_hash() {
    assert_eq!(hash("HASH"), 52);
}

#[test]
fn test_part1() {
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    assert_eq!(part1(input), 1320);
}

#[test]
fn test_part2() {
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    assert_eq!(part2(input), 145);
}

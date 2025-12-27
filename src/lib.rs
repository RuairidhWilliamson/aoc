pub mod ascii_grid;
pub mod grid;
pub mod integer_iter;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;

use std::{
    io::ErrorKind,
    num::NonZero,
    path::Path,
    time::{Duration, Instant},
};

pub struct Config {
    pub day_filter: Option<u32>,
    pub part_filter: Option<u8>,
    pub count: NonZero<u32>,
    pub input_dir: &'static Path,
    pub bless_snapshots: bool,
    pub snapshot_dir: &'static Path,
    pub sucess: bool,
    pub elapsed: Duration,
}

impl Config {
    pub fn new_from_env() -> Self {
        let day_filter = std::env::var("DAY").ok().map(|d| d.parse::<u32>().unwrap());
        let part_filter = std::env::var("PART").ok().map(|p| p.parse::<u8>().unwrap());
        let use_testdata = env_is_enabled("TESTDATA");
        let bless_snapshots = env_is_enabled("BLESS");
        let count = std::env::var("COUNT")
            .ok()
            .map(|c| c.parse::<NonZero<u32>>().unwrap())
            .unwrap_or(NonZero::new(1).unwrap());
        let input_dir = if use_testdata {
            Path::new("test_input")
        } else {
            Path::new("input")
        };
        let snapshot_dir = if use_testdata {
            Path::new("test_snapshots")
        } else {
            Path::new("snapshots")
        };
        std::fs::create_dir_all(snapshot_dir).unwrap();
        Self {
            day_filter,
            part_filter,
            count,
            input_dir,
            bless_snapshots,
            snapshot_dir,
            sucess: true,
            elapsed: Duration::ZERO,
        }
    }

    pub fn load_input<const PART: u8>(&self, string_day: &str) -> String {
        let res = std::fs::read_to_string(self.input_dir.join(string_day.to_owned() + ".txt"));
        match res {
            Ok(contents) => contents,
            Err(err) if err.kind() == ErrorKind::NotFound => std::fs::read_to_string(
                self.input_dir
                    .join(format!("{}_part{}.txt", string_day.to_owned(), PART)),
            )
            .unwrap(),
            Err(err) => {
                panic!("{}", err)
            }
        }
    }

    pub fn run_part<const PART: u8, F, T>(&mut self, part_fn: F, input: &str, string_day: &str)
    where
        F: Fn(&str) -> T,
        T: std::fmt::Debug + Eq + std::fmt::Display,
    {
        if self.part_filter.is_none_or(|p| p == PART) {
            let timer = Instant::now();
            let result = part_fn(input);
            for _ in 1..self.count.get() {
                let r = part_fn(input);
                assert_eq!(result, r);
            }
            let elapsed = timer.elapsed() / self.count.get();
            self.elapsed += elapsed;
            println!(" Part {PART} = {result}  Elapsed {elapsed:?}");
            let snapshot_name = format!("{string_day}_part{PART}.txt");
            let snapshot_path = self.snapshot_dir.join(snapshot_name);
            if self.bless_snapshots {
                std::fs::write(&snapshot_path, result.to_string()).unwrap();
            } else {
                match std::fs::read_to_string(&snapshot_path) {
                    Ok(snapshot) => {
                        if snapshot != result.to_string() {
                            println!(" Snapshot did not match!!!");
                            self.sucess = false;
                        }
                    }
                    Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                        println!(" Snapshot not set");
                    }
                    Err(err) => panic!("{err:?}"),
                }
            }
        }
    }
}

pub fn env_is_enabled(name: &str) -> bool {
    std::env::var(name).is_ok_and(|v| {
        v != "0" && !v.eq_ignore_ascii_case("false") && !v.eq_ignore_ascii_case("no")
    })
}

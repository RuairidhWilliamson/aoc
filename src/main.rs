mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

use std::{
    path::Path,
    process::ExitCode,
    sync::atomic::{AtomicBool, Ordering},
    time::Instant,
};

macro_rules! day {
    ($config:ident, $module:ident, $day:expr) => {
        let string_day = stringify!($module);
        if $config.day_filter.is_none_or(|d| d == $day) {
            println!("Day {}", $day);
            let input =
                std::fs::read_to_string($config.input_dir.join(string_day.to_owned() + ".txt"))
                    .unwrap();
            $config.run_part::<1, _, _>($module::part1, &input, string_day);
            $config.run_part::<2, _, _>($module::part2, &input, string_day);
        }
    };
}

struct Config {
    day_filter: Option<u32>,
    part_filter: Option<u8>,
    input_dir: &'static Path,
    bless_snapshots: bool,
    snapshot_dir: &'static Path,
    sucess: AtomicBool,
}

impl Config {
    fn run_part<const PART: u8, F: FnOnce(&str) -> T, T: std::fmt::Display>(
        &self,
        part_fn: F,
        input: &str,
        string_day: &str,
    ) {
        if self.part_filter.is_none_or(|p| p == PART) {
            let timer = Instant::now();
            let result = part_fn(input);
            let elapsed = timer.elapsed();
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
                            self.sucess.store(false, Ordering::Relaxed);
                        }
                    }
                    Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
                    Err(err) => panic!("{err:?}"),
                }
            }
        }
    }
}

fn main() -> ExitCode {
    let day_filter = std::env::var("DAY").ok().map(|d| d.parse::<u32>().unwrap());
    let part_filter = std::env::var("PART").ok().map(|p| p.parse::<u8>().unwrap());
    let use_testdata = env_is_enabled("TESTDATA");
    let bless_snapshots = env_is_enabled("BLESS");
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
    let config = &Config {
        day_filter,
        part_filter,
        input_dir,
        bless_snapshots,
        snapshot_dir,
        sucess: AtomicBool::new(true),
    };

    day!(config, day01, 1);
    day!(config, day02, 2);
    day!(config, day03, 3);
    day!(config, day04, 4);
    day!(config, day05, 5);
    day!(config, day06, 6);

    if config.sucess.load(Ordering::Relaxed) {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn env_is_enabled(name: &str) -> bool {
    std::env::var(name).is_ok_and(|v| {
        v != "0" && !v.eq_ignore_ascii_case("false") && !v.eq_ignore_ascii_case("no")
    })
}

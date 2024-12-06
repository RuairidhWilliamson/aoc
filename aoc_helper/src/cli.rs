use std::{
    num::{NonZeroUsize, ParseIntError},
    process::ExitCode,
    str::FromStr,
};

use yansi::Paint as _;

#[macro_export]
macro_rules! days {
    ($($day:ident,)*) => {
        $(pub mod $day;)*
        pub static DAYS: &$crate::cli::DaysList = &[
            $(($day::solve_part1, $day::solve_part2),)*
        ];
    }
}

pub type SolveFn = fn(&str) -> usize;
pub type DaysList = [(SolveFn, SolveFn)];

pub fn run(year: usize, days: &DaysList) -> ExitCode {
    match run_inner(year, days) {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE,
    }
}

fn run_inner(year: usize, days: &DaysList) -> Result<(), ()> {
    let mut args = std::env::args();
    // skip process arg
    args.next().ok_or_else(|| {
        eprintln!("could not find process arg");
    })?;

    let res = if let Some(s) = args.next() {
        let selection: Selection = s.parse().map_err(|err: Error| {
            eprintln!("{}", err.red());
            print_usage();
        })?;
        selection.run(year, days)
    } else {
        Selection::run_all(year, days)
    };

    res.map_err(|err: Error| {
        eprintln!("{}", err.red());
        print_usage();
    })
}

#[derive(Debug)]
enum Error {
    DayOutOfRange,
    DayDoesNotExistYet,
    DayParse,
    DayParseNum(ParseIntError),
    Part,
    GetInput(crate::downloader::Error),
}

impl From<crate::downloader::Error> for Error {
    fn from(err: crate::downloader::Error) -> Self {
        Self::GetInput(err)
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DayOutOfRange => f.write_str("day must be in the range 1 - 25"),
            Self::DayDoesNotExistYet => f.write_str("day does not exist yet"),
            Self::DayParse => f.write_str("could not parse day"),
            Self::DayParseNum(err) => f.write_fmt(format_args!("could not parse day: {err}")),
            Self::Part => f.write_str("could not parse part, part can only be 1 or 2"),
            Self::GetInput(err) => f.write_fmt(format_args!("download error: {err}")),
        }
    }
}

struct Selection {
    day: NonZeroUsize,
    part: Option<Part>,
}

impl FromStr for Selection {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.splitn(2, ':');
        let day: NonZeroUsize = iter
            .next()
            .ok_or(Error::DayParse)?
            .parse()
            .map_err(Error::DayParseNum)?;
        if day.get() > 25 {
            return Err(Error::DayOutOfRange);
        }
        let part = iter.next().map(Part::from_str).transpose()?;
        Ok(Self { day, part })
    }
}

impl Selection {
    fn run_all(year: usize, days: &DaysList) -> Result<(), Error> {
        for d in 1..=days.len() {
            Self {
                day: NonZeroUsize::new(d).expect("DAYS array is empty"),
                part: None,
            }
            .run(year, days)?;
        }
        Ok(())
    }

    fn run(&self, year: usize, days: &DaysList) -> Result<(), Error> {
        if self.day.get() > days.len() {
            return Err(Error::DayDoesNotExistYet);
        }
        let input = crate::downloader::get_input(year, self.day)?;
        match self.part {
            Some(Part::One) => self.run_part1(&input, days),
            Some(Part::Two) => self.run_part2(&input, days),
            None => {
                self.run_part1(&input, days);
                self.run_part2(&input, days);
            }
        }
        Ok(())
    }

    fn run_part1(&self, input: &str, days: &DaysList) {
        let (part1, _) = &days[self.day.get() - 1];
        let out = part1(input);
        println!("{:02}:1 => {}", self.day.get(), out.blue());
    }

    fn run_part2(&self, input: &str, days: &DaysList) {
        let (_, part2) = &days[self.day.get() - 1];
        let out = part2(input);
        println!("{:02}:2 => {}", self.day.get(), out.blue());
    }
}

enum Part {
    One,
    Two,
}

impl FromStr for Part {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Self::One),
            "2" => Ok(Self::Two),
            _ => Err(Error::Part),
        }
    }
}

fn print_usage() {
    eprintln!("Usage: {}", "aoc [day][:part]".yellow());
    eprintln!("{}", "\t where day is a number 1-25".blue());
    eprintln!("{}", "\t where part is a 1 or 2".blue());
}

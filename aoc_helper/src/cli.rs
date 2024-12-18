use std::{
    num::{NonZeroUsize, ParseIntError},
    process::ExitCode,
    str::FromStr,
    time::Instant,
};

use yansi::Paint as _;

#[macro_export]
macro_rules! days {
    ($($day:ident,)*) => {
        $(pub mod $day;)*

        pub fn days() -> $crate::cli::DaysList {
            vec![
                $(($crate::cli::SolveFnMagic::new($day::solve_part1), $crate::cli::SolveFnMagic::new($day::solve_part2)),)*
            ]
        }
    }
}

pub type SolveFn = fn(&str) -> usize;

pub struct SolveFnMagic(Box<InnerFn>);

type InnerFn = dyn Fn(&str) -> Box<dyn std::fmt::Display>;

impl SolveFnMagic {
    pub fn new<T: std::fmt::Display + 'static>(f: impl (Fn(&str) -> T) + 'static) -> Self {
        Self(Box::new(move |input: &str| -> Box<dyn std::fmt::Display> {
            Box::new(f(input))
        }))
    }

    fn run(&self, input: &str) -> Box<dyn std::fmt::Display> {
        self.0(input)
    }
}

pub type DaysList = Vec<(SolveFnMagic, SolveFnMagic)>;

#[must_use]
pub fn run(year: usize, days: &[(SolveFnMagic, SolveFnMagic)]) -> ExitCode {
    match run_inner(year, days) {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE,
    }
}

fn run_inner(year: usize, days: &[(SolveFnMagic, SolveFnMagic)]) -> Result<(), ()> {
    let mut args = std::env::args();
    // skip process arg
    args.next().ok_or_else(|| {
        eprintln!("could not find process arg");
    })?;

    let start = Instant::now();
    let res = if let Some(s) = args.next() {
        let selection: Selection = s.parse().map_err(|err: Error| {
            eprintln!("{}", err.red());
            print_usage();
        })?;
        selection.run(year, days)
    } else {
        Selection::run_all(year, days)
    };
    let elapsed = start.elapsed().as_millis();
    println!("Total Elapsed {elapsed} ms");

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
    fn run_all(year: usize, days: &[(SolveFnMagic, SolveFnMagic)]) -> Result<(), Error> {
        for d in 1..=days.len() {
            Self {
                day: NonZeroUsize::new(d).expect("DAYS array is empty"),
                part: None,
            }
            .run(year, days)?;
        }
        Ok(())
    }

    fn run(&self, year: usize, days: &[(SolveFnMagic, SolveFnMagic)]) -> Result<(), Error> {
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

    fn run_part1(&self, input: &str, days: &[(SolveFnMagic, SolveFnMagic)]) {
        let (part1, _) = &days[self.day.get() - 1];
        self.run_part(input, 1, part1);
    }

    fn run_part2(&self, input: &str, days: &[(SolveFnMagic, SolveFnMagic)]) {
        let (_, part2) = &days[self.day.get() - 1];
        self.run_part(input, 2, part2);
    }

    fn run_part(&self, input: &str, part: usize, part_fun: &SolveFnMagic) {
        let start = Instant::now();
        let out = part_fun.run(input);
        let elapsed = start.elapsed().as_millis();
        let day = self.day.get();
        let out = out.blue();
        println!("{day:02}:{part} => {out:>20}\t\t{elapsed:>7} ms");
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

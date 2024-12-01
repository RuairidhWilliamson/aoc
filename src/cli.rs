use std::{
    fs::create_dir_all,
    io::ErrorKind,
    num::{NonZeroUsize, ParseIntError},
    path::Path,
    str::FromStr,
};

use reqwest::{header::COOKIE, StatusCode};
use yansi::Paint;

pub fn cli() -> Result<(), ()> {
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
        selection.run()
    } else {
        Selection::run_all()
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
    ReadInput(std::io::Error),
    ReadSecret(std::io::Error),
    ReqwestFailed(reqwest::Error),
    DownloadHTTP(StatusCode, String),
    DownloadIO(std::io::Error),
    BadPath,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DayOutOfRange => f.write_str("day must be in the range 1 - 25"),
            Error::DayDoesNotExistYet => f.write_str("day does not exist yet"),
            Error::DayParse => f.write_str("could not parse day"),
            Error::DayParseNum(err) => f.write_fmt(format_args!("could not parse day: {err}")),
            Error::Part => f.write_str("could not parse part, part can only be 1 or 2"),
            Error::ReadInput(err) => f.write_fmt(format_args!("could not read input: {err}")),
            Error::ReadSecret(err) => {
                f.write_fmt(format_args!("could not read secret file: {err}"))
            }
            Error::ReqwestFailed(err) => {
                f.write_fmt(format_args!("could not perform reqwest: {err}"))
            }
            Error::DownloadHTTP(status_code, text) => {
                f.write_fmt(format_args!("download input failed {status_code}: {text}"))
            }
            Error::DownloadIO(err) => f.write_fmt(format_args!("download io error: {err}")),
            Error::BadPath => f.write_str("bad path"),
        }
    }
}

struct Selection {
    day: NonZeroUsize,
    part: Option<Part>,
}

impl FromStr for Selection {
    type Err = Error;

    fn from_str(s: &str) -> Result<Selection, Self::Err> {
        let mut iter = s.splitn(2, ':');
        let day: NonZeroUsize = iter
            .next()
            .ok_or(Error::DayParse)?
            .parse()
            .map_err(Error::DayParseNum)?;
        if day.get() > 25 {
            return Err(Error::DayOutOfRange);
        }
        if day.get() > super::DAYS.len() {
            return Err(Error::DayDoesNotExistYet);
        }
        let part = iter.next().map(Part::from_str).transpose()?;
        Ok(Self { day, part })
    }
}

impl Selection {
    fn run_all() -> Result<(), Error> {
        for d in 1..=super::DAYS.len() {
            Self {
                day: NonZeroUsize::new(d).expect("DAYS array is empty"),
                part: None,
            }
            .run()?;
        }
        Ok(())
    }

    fn get_input(&self) -> Result<String, Error> {
        let input_name = format!("day{:02}.txt", self.day.get());
        let input_path = Path::new("inputs").join(input_name);
        match std::fs::read_to_string(&input_path) {
            Ok(s) => {
                return Ok(s);
            }
            Err(err) if err.kind() == ErrorKind::NotFound => {
                download_input(self.day, &input_path)?;
            }
            Err(err) => {
                return Err(Error::ReadInput(err));
            }
        }
        // Retry now that we downloaded the input
        match std::fs::read_to_string(&input_path) {
            Ok(s) => Ok(s),
            Err(err) => Err(Error::ReadInput(err)),
        }
    }

    fn run(&self) -> Result<(), Error> {
        let input = self.get_input()?;
        match self.part {
            Some(Part::One) => self.run_part1(&input),
            Some(Part::Two) => self.run_part2(&input),
            None => {
                self.run_part1(&input);
                self.run_part2(&input);
            }
        }
        Ok(())
    }

    fn run_part1(&self, input: &str) {
        // println!("Running day {} part 1...", self.day.get());
        let (part1, _) = super::DAYS[self.day.get() - 1];
        let out = part1(input);
        println!("Day {} part 1 answer is {out}", self.day.get());
    }

    fn run_part2(&self, input: &str) {
        // println!("Running day {} part 2...", self.day.get());
        let (_, part2) = super::DAYS[self.day.get() - 1];
        let out = part2(input);
        println!("Day {} part 2 answer is {out}", self.day.get());
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

fn download_input(day: NonZeroUsize, path: &Path) -> Result<(), Error> {
    println!("Downloading input for day {day}");
    let day = day.get();
    let client = reqwest::blocking::Client::new();
    let url = format!("https://adventofcode.com/2024/day/{day}/input");
    let req = client
        .get(url)
        .header(COOKIE, format!("session={}", get_session_id()?));
    let mut response = req.send().map_err(Error::ReqwestFailed)?;
    let status = response.status();
    if status != 200 {
        return Err(Error::DownloadHTTP(
            status,
            response.text().unwrap_or_default(),
        ));
    }
    let input_directory = path.parent().ok_or(Error::BadPath)?;
    create_dir_all(input_directory).map_err(Error::DownloadIO)?;
    let mut out_file = std::fs::File::create_new(path).map_err(Error::DownloadIO)?;
    std::io::copy(&mut response, &mut out_file).map_err(Error::DownloadIO)?;
    Ok(())
}

fn get_session_id() -> Result<String, Error> {
    Ok(std::fs::read_to_string("secret.txt")
        .map_err(Error::ReadSecret)?
        .trim()
        .to_owned())
}

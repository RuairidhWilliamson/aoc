use std::{io::ErrorKind, num::NonZeroUsize, path::Path};

use reqwest::StatusCode;

#[derive(Debug)]
pub enum Error {
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
            Self::ReadInput(err) => f.write_fmt(format_args!("could not read input: {err}")),
            Self::ReadSecret(err) => {
                f.write_fmt(format_args!("could not read secret file secret.txt: {err}"))
            }
            Self::ReqwestFailed(err) => {
                f.write_fmt(format_args!("could not perform reqwest: {err}"))
            }
            Self::DownloadHTTP(status_code, text) => {
                f.write_fmt(format_args!("download input failed {status_code}: {text}"))
            }
            Self::DownloadIO(err) => f.write_fmt(format_args!("download io error: {err}")),
            Self::BadPath => f.write_str("bad path"),
        }
    }
}

pub fn get_input(year: usize, day: NonZeroUsize) -> Result<String, Error> {
    let input_name = format!("day{:02}.txt", day.get());
    let input_path = Path::new("inputs").join(input_name);
    match std::fs::read_to_string(&input_path) {
        Ok(s) => {
            return Ok(s);
        }
        Err(err) if err.kind() == ErrorKind::NotFound => {
            download_input(year, day, &input_path)?;
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

fn download_input(year: usize, day: NonZeroUsize, path: &Path) -> Result<(), Error> {
    println!("Downloading input for day {day}");
    let day = day.get();
    let client = reqwest::blocking::Client::new();
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let req = client.get(url).header(
        reqwest::header::COOKIE,
        format!("session={}", get_session_id()?),
    );
    let mut response = req.send().map_err(Error::ReqwestFailed)?;
    let status = response.status();
    if status != 200 {
        return Err(Error::DownloadHTTP(
            status,
            response.text().unwrap_or_default(),
        ));
    }
    let input_directory = path.parent().ok_or(Error::BadPath)?;
    std::fs::create_dir_all(input_directory).map_err(Error::DownloadIO)?;
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

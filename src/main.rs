use std::process::ExitCode;

mod cli;

type SolveFn = fn(&str) -> usize;

macro_rules! days {
    ($($day:ident,)*) => {
        $(mod $day;)*
        static DAYS: &[(SolveFn, SolveFn)] = &[
            $(($day::solve_part1, $day::solve_part2),)*
        ];
    }
}

days! {
    day01,
    day02,
}

fn main() -> ExitCode {
    match cli::cli() {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE,
    }
}

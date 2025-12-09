use std::process::ExitCode;

macro_rules! day {
    ($config:ident, $module:ident, $day:expr) => {
        let string_day = stringify!($module);
        if $config.day_filter.is_none_or(|d| d == $day) {
            println!("Day {}", $day);
            let input = $config.load_input(string_day);
            $config.run_part::<1, _, _>(aoc::$module::part1, &input, string_day);
            $config.run_part::<2, _, _>(aoc::$module::part2, &input, string_day);
        }
    };
}

fn main() -> ExitCode {
    let mut config = aoc::Config::new_from_env();

    day!(config, day01, 1);
    day!(config, day02, 2);
    day!(config, day03, 3);
    day!(config, day04, 4);
    day!(config, day05, 5);
    day!(config, day06, 6);
    day!(config, day07, 7);
    day!(config, day08, 8);
    day!(config, day09, 9);

    println!("Total Elapsed {:?}", config.elapsed);

    if config.sucess {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

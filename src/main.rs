use aoc::utils::Part;
use aoc::year::DayParts;
use clap::{Parser, Subcommand};
use color_eyre::{Report, eyre::eyre};
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use tracing::debug;
use tracing_subscriber::EnvFilter;
mod y2015;
mod y2016;
mod y2022;
mod y2023;
mod y2024;
use aoc::year::Year;

#[derive(Debug, Subcommand)]
enum Action {
    Run {
        /// File input that contains the source puzzel data
        #[arg(long)]
        input: Option<String>,
        /// the part of the task
        #[arg(value_enum, long, default_value = "one")]
        part: Part,
    },
    Create {
        /// File input that contains the source puzzel data
        #[arg(required = true)]
        template: PathBuf,
    },
}

#[derive(Parser, Debug)]
#[command(name = "aoc", author = "iatanaso")]
struct Cli {
    #[command(subcommand)]
    subcommand: Action,
    #[arg(value_enum, long, short, default_value = "info")]
    log: tracing::Level,
    #[arg(short, long, required = true)]
    /// The day of the Advent of code
    day: usize,
    #[arg(short, long, default_value = "y2024")]
    /// The year by default that most recent year
    year: Year,
}

fn setup(args: &Cli) -> Result<(), Report> {
    color_eyre::install()?;
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_max_level(args.log)
        .init();
    Ok(())
}

fn main() -> Result<(), Report> {
    let cli = Cli::parse();
    setup(&cli)?;
    match cli.subcommand {
        Action::Run { input, part } => {
            let content = if let Some(file) = input {
                let path = PathBuf::from_str(&file)?;
                if path.is_file() {
                    fs::read_to_string(path)?
                } else {
                    file
                }
            } else {
                String::from("")
            };
            match cli.year {
                Year::Y2022 => match cli.day {
                    1 => y2022::day1::execute(content, part),
                    2 => y2022::day2::execute(content, part),
                    3 => y2022::day3::execute(content, part),
                    4 => y2022::day4::execute(content, part),
                    5 => y2022::day5::execute(content, part),
                    6 => y2022::day6::execute(content, part),
                    7 => y2022::day7::execute(content, part),
                    8 => y2022::day8::execute(content, part),
                    9 => y2022::day9::execute(content, part),
                    10 => y2022::day10::execute(content, part),
                    //"9" => day9(content),
                    _ => Err(eyre!("Error Not Implemented")),
                },
                Year::Y2023 => match cli.day {
                    1 => y2023::day1::execute(content, part),
                    2 => y2023::day2::execute(content, part),
                    3 => y2023::day3::execute(content, part),
                    4 => y2023::day4::execute(content, part),
                    5 => y2023::day5::execute(content, part),
                    6 => y2023::day6::execute(content, part),
                    7 => y2023::day7::execute(content, part),
                    8 => y2023::day8::execute(content, part),
                    9 => y2023::day9::execute(content, part),
                    10 => y2023::day10::execute(content, part),
                    11 => y2023::day11::execute(content, part),
                    12 => y2023::day12::execute(content, part),
                    13 => y2023::day13::execute(content, part),
                    14 => y2023::day14::execute(content, part),
                    15 => y2023::day15::execute(content, part),
                    16 => y2023::day16::execute(content, part),
                    _ => Err(eyre!("Error Not Implemented")),
                },
                Year::Y2024 => match cli.day {
                    1 => y2024::day1::execute(content, part),
                    2 => y2024::day2::execute(content, part),
                    3 => y2024::day3::execute(content, part),
                    4 => y2024::day4::execute(content, part),
                    5 => y2024::day5::execute(content, part),
                    6 => y2024::day6::execute(content, part),
                    7 => y2024::day7::execute(content, part),
                    8 => y2024::day8::execute(content, part),
                    9 => y2024::day9::execute(content, part),
                    _ => Err(eyre!("Error not Implemented")),
                },
                Year::Y2015 => match cli.day {
                    1 => y2015::Day1 { content }.execute(part),
                    2 => y2015::Day2 { content }.execute(part),
                    3 => y2015::Day3 { content }.execute(part),
                    4 => y2015::Day4 { content }.execute(part),
                    5 => y2015::Day5 { content }.execute(part),
                    6 => y2015::Day6 { content }.execute(part),
                    7 => y2015::Day7 { content }.execute(part),
                    8 => y2015::Day8 { content }.execute(part),
                    9 => y2015::Day9 { content }.execute(part),
                    10 => y2015::Day10 { content }.execute(part),
                    11 => y2015::Day11 { content }.execute(part),
                    12 => y2015::Day12 { content }.execute(part),
                    _ => Err(eyre!("Error not Implemented")),
                },
                Year::Y2016 => match cli.day {
                    1 => y2016::Day1 { content }.execute(part),
                    _ => Err(eyre!("Error not Implemented")),
                },
                _ => Err(eyre!("Error not Implemented")),
            }
        }
        Action::Create { template } => {
            let template = fs::read_to_string(template)?;
            let location = PathBuf::from_str(&format!("src/{}/mod.rs", cli.year.to_src()))?;
            if location.exists() {
                return Err(eyre!("Location Exists {}", location.to_str().unwrap()));
            }
            fs::write(location, template.as_bytes())?;

            let mod_location = format!("src/{}/mod.rs", cli.year.to_src());
            let mod_rs = fs::read_to_string(&mod_location)?;
            let new_mod_rs = format!("{}pub mod day{};", mod_rs, cli.day);
            fs::write(mod_location, new_mod_rs.as_bytes())?;
            debug!(new_mod_rs);
            Ok(())
        }
    }
}

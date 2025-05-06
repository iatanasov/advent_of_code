use aoc::utils::Part;
use clap::{Parser, ValueEnum};
use color_eyre::{Report, eyre::eyre};
use std::fs;
use std::path::PathBuf;
use tracing_subscriber::EnvFilter;
mod y2015;
mod y2016;
mod y2017;
mod y2018;
mod y2019;
mod y2020;
mod y2021;
mod y2022;
mod y2023;
mod y2024;

#[derive(Debug, Clone, ValueEnum)]
enum Year {
    Y2015,
    Y2016,
    Y2017,
    Y2018,
    Y2019,
    Y2020,
    Y2021,
    Y2022,
    Y2023,
    Y2024,
}
#[derive(Parser, Debug)]
#[command(name = "aoc", author = "iatanaso")]
struct Cli {
    #[arg(short, long, required = true)]
    /// The day of the Advent of code
    day: usize,
    #[arg(short, long, default_value = "y2024")]
    /// The year by default that most recent year
    year: Year,
    /// File input that contains the source puzzel data
    #[arg(required = true)]
    input: PathBuf,
    /// the part of the task
    #[arg(value_enum, long, default_value = "one")]
    part: Part,
    #[arg(value_enum, long, default_value = "info")]
    log: tracing::Level,
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
    let args = Cli::parse();
    setup(&args)?;
    let file = args.input;
    let content = fs::read_to_string(file)?;
    match args.year {
        Year::Y2022 => match args.day {
            1 => y2022::day1::execute(content, args.part),
            2 => y2022::day2::execute(content, args.part),
            3 => y2022::day3::execute(content, args.part),
            4 => y2022::day4::execute(content, args.part),
            5 => y2022::day5::execute(content, args.part),
            6 => y2022::day6::execute(content, args.part),
            7 => y2022::day7::execute(content, args.part),
            8 => y2022::day8::execute(content, args.part),
            9 => y2022::day9::execute(content, args.part),
            10 => y2022::day10::execute(content, args.part),
            //"9" => day9(content),
            _ => Err(eyre!("Error Not Implemented")),
        },
        Year::Y2023 => match args.day {
            1 => y2023::day1::execute(content, args.part),
            2 => y2023::day2::execute(content, args.part),
            3 => y2023::day3::execute(content, args.part),
            4 => y2023::day4::execute(content, args.part),
            5 => y2023::day5::execute(content, args.part),
            6 => y2023::day6::execute(content, args.part),
            7 => y2023::day7::execute(content, args.part),
            8 => y2023::day8::execute(content, args.part),
            9 => y2023::day9::execute(content, args.part),
            10 => y2023::day10::execute(content, args.part),
            11 => y2023::day11::execute(content, args.part),
            12 => y2023::day12::execute(content, args.part),
            13 => y2023::day13::execute(content, args.part),
            14 => y2023::day14::execute(content, args.part),
            15 => y2023::day15::execute(content, args.part),
            16 => y2023::day16::execute(content, args.part),
            _ => Err(eyre!("Error Not Implemented")),
        },
        Year::Y2024 => match args.day {
            1 => y2024::day1::execute(content, args.part),
            2 => y2024::day2::execute(content, args.part),
            3 => y2024::day3::execute(content, args.part),
            4 => y2024::day4::execute(content, args.part),
            5 => y2024::day5::execute(content, args.part),
            6 => y2024::day6::execute(content, args.part),
            7 => y2024::day7::execute(content, args.part),
            8 => y2024::day8::execute(content, args.part),
            9 => y2024::day9::execute(content, args.part),
            _ => Err(eyre!("Error not Implemented")),
        },
        Year::Y2015 => match args.day {
            1 => y2015::day1::execute(content, args.part),
            _ => Err(eyre!("Error not Implemented")),
        },
        Year::Y2016 => match args.day {
            1 => y2016::day1::execute(content, args.part),
            _ => Err(eyre!("Error not Implemented")),
        },
        Year::Y2017 => match args.day {
            1 => y2017::day1::execute(content, args.part),
            _ => Err(eyre!("Error not Implemented")),
        },
        Year::Y2018 => match args.day {
            1 => y2018::day1::execute(content, args.part),
            _ => Err(eyre!("Error not Implemented")),
        },
        Year::Y2019 => match args.day {
            1 => y2019::day1::execute(content, args.part),
            _ => Err(eyre!("Error not Implemented")),
        },
        Year::Y2020 => match args.day {
            1 => y2020::day1::execute(content, args.part),
            _ => Err(eyre!("Error not Implemented")),
        },
        Year::Y2021 => match args.day {
            1 => y2021::day1::execute(content, args.part),
            _ => Err(eyre!("Error not Implemented")),
        },
    }
}

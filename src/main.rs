use clap::Parser;
use color_eyre::{eyre::eyre, Report};
use std::fs;
use std::path::PathBuf;
use tracing_subscriber::EnvFilter;
mod y2022;
mod y2023;
mod y2024;
use aoc_2022::utils::{LocalLogLevel, Part};

#[derive(Parser, Debug)]
#[command(name = "aoc", author = "iatanaso")]
struct Cli {
    #[arg(short, long, required = true)]
    /// The day of the Advent of code
    day: usize,
    #[arg(short, long, default_value = "2024")]
    /// The year by default that most recent year
    year: usize,
    /// File input that contains the source puzzel data
    #[arg(required = true)]
    input: PathBuf,
    /// the part of the task
    #[arg(value_enum, long, default_value = "one")]
    part: Part,
    #[arg(value_enum, long, default_value = "info")]
    log: LocalLogLevel,
}

fn setup(args: &Cli) -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{}", &args.log));
    }

    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_ansi(false)
        .init();
    Ok(())
}

fn main() -> Result<(), Report> {
    let args = Cli::parse();
    setup(&args)?;
    let file = args.input;
    let content = fs::read_to_string(file)?;
    match args.year {
        2022 => {
            match args.day {
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
            }
        }
        2023 => match args.day {
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
        2024 => match args.day {
            1 => y2024::day1::execute(content, args.part),
            2 => y2024::day2::execute(content, args.part),
            3 => y2024::day2::execute(content, args.part),
            _ => Err(eyre!("Error not Implemented")),
        },
        _ => Ok(()),
    }
}

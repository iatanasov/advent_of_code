use std::fs;
use std::path::PathBuf;
use clap::{Parser};
use color_eyre::{eyre::eyre,Report};
use tracing_subscriber;
use tracing_subscriber::EnvFilter;
mod y2022;
use aoc_2022::utils::{Part, LocalLogLevel};

#[derive(Parser, Debug)]
#[command(name = "aoc", author = "iatanaso")]
struct Cli {
    #[arg(short, long, required = true)]
    day: usize,
    #[arg(short, long, default_value = "2022")]
    year: usize,
    #[arg(required=true)]
    input: PathBuf,
    #[arg(value_enum, long, default_value="one")]
    part: Part,
    #[arg(value_enum, long, default_value="info")]
    log: LocalLogLevel
}


fn setup(args: &Cli) -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE","1")
    }
    color_eyre::install()?;
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG",format!("{}",&args.log));
    }

    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    Ok(())
}

fn main() -> Result<(),Report> { 
    let args = Cli::parse();
    setup(&args)?;
    let file = args.input;
    let content = fs::read_to_string(file)?;
    return match args.year {
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
                _ => return Err(eyre!("Error Not Implemented"))
            }
        }
        _ => return Ok(())
    }}

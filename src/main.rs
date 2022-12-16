use std::fs;
use std::path::PathBuf;
use clap::{Parser};
use color_eyre::Report;
use tracing_subscriber;
use tracing_subscriber::EnvFilter;
mod y2022;
use aoc_2022::utils::Part;

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
    part: Part
}


fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE","1")
    }
    color_eyre::install()?;
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG","info")
    }

    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    Ok(())
}
fn main() -> Result<(),Report> {
    setup()?;
    let args = Cli::parse();
    let file = args.input;
    let content = fs::read_to_string(file)?;
    return match args.year {
        2022 => {
            match args.day {
                1 => y2022::day1::execute(content, args.part),
                2 => y2022::day2::execute(content, args.part),
                3 => y2022::day3::execute(content, args.part),
                //"4" => days::day4::execute(content),
                //"5" => days::day5::execute(content),
                //"6" => days::day6::execute(content),
                //"7" => days::day7::execute(content),
                //"8" => days::day8::execute(content),
                //"9" => day9(content),
                _ => return Ok(())
            }
        }
        _ => return Ok(())
    }
}


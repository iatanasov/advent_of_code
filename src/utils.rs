use clap::ValueEnum;
use color_eyre::Report;
use color_eyre::eyre::eyre;
use std::fmt;

#[derive(ValueEnum, Clone, Debug, Copy)]
pub enum Part {
    One,
    Two
}

#[derive(ValueEnum, Clone, Debug, Copy)]
pub enum LocalLogLevel {
   Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl fmt::Display for LocalLogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LocalLogLevel::Error => write!(f,"error"),
            LocalLogLevel::Warn => write!(f,"warn"),
            LocalLogLevel::Info => write!(f,"info"),
            LocalLogLevel::Debug => write!(f,"debug"),
            LocalLogLevel::Trace => write!(f,"trace"),
        }
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
struct Day  {
    content: String,
    part: Part,
}

impl SolveDay for Day {
    fn part(&self) -> Part {
        return self.part
    }
}

trait SolveDay {
    
    fn part(&self) -> Part;

    fn execute(&self) -> Result<(), Report> {
        match self.part() {
            Part::One => self.part1(),
            Part::Two => self.part2(),
        }
    }

    fn part1(&self) -> Result<(), Report> {
        Err(eyre!("Part 1 is not implemented"))
    }
    fn part2(&self) -> Result<(), Report>{ 
        Err(eyre!("Part 2 is not implemented"))
    }
}

pub fn to_digits(chars: &Vec<char>) -> usize {
    return format!("{}",chars.iter().collect::<String>()).parse::<usize>().unwrap();
}
pub fn str_to_digits<S: Into<String>>(s: S) -> usize {
    return format!("{}",s.into()).parse::<usize>().unwrap();
}

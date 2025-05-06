use clap::ValueEnum;
use color_eyre::Report;
use color_eyre::eyre::eyre;

#[derive(ValueEnum, Clone, Debug, Copy)]
pub enum Part {
    One,
    Two,
}
#[allow(dead_code)]
#[allow(unused_variables)]
struct Day {
    content: String,
    part: Part,
}

impl SolveDay for Day {
    fn part(&self) -> Part {
        self.part
    }
}
#[allow(dead_code)]
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
    fn part2(&self) -> Result<(), Report> {
        Err(eyre!("Part 2 is not implemented"))
    }
}

pub fn to_digits(chars: &[char]) -> usize {
    chars
        .iter()
        .collect::<String>()
        .to_string()
        .parse::<usize>()
        .unwrap()
}
pub fn str_to_digits<S: Into<String>>(s: S) -> usize {
    s.into().to_string().parse::<usize>().unwrap()
}
pub fn str_to_isize<S: Into<String>>(s: S) -> isize {
    s.into().to_string().parse::<isize>().unwrap()
}

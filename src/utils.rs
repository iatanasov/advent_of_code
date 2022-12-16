use clap::ValueEnum;
use color_eyre::Report;
use color_eyre::eyre::eyre;

#[derive(ValueEnum, Clone, Debug, Copy)]
pub enum Part {
    One,
    Two
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

//fn dowork() {
//    let d = Day { content: String::from("aaa"), part: Part::One };
//    SolveDay::execute(&d);
///}

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

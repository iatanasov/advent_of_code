use aoc::year::DayParts;
use color_eyre::Report;

pub struct Day1 {
    pub content: String,
}
impl DayParts for Day1 {
    fn part1(&self) -> Result<(), Report> {
        println!("{}", self.content);
        Ok(())
    }
    fn part2(&self) -> Result<(), Report> {
        Ok(())
    }
}

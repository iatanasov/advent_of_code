#![allow(dead_code)]
#![allow(unused_variables)]
use aoc::utils::Part;
use color_eyre::Report;
use tracing::info;

pub fn execute(content: String, part: Part) -> Result<(), Report> {
    info!("Running {:?} ", part);
    match part {
        Part::One => part1(content),
        Part::Two => part2(content),
    }
}

#[allow(unused_variables)]
pub fn part1(content: String) -> Result<(), Report> {
    Ok(())
}
#[allow(unused_variables)]
pub fn part2(content: String) -> Result<(), Report> {
    Ok(())
}

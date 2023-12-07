#![allow(dead_code)]
#![allow(unused_variables)]
use tracing::info;
use color_eyre::Report;
use aoc_2022::utils::Part;


pub fn execute(content: String, part: Part) -> Result<(), Report> {
    info!("Running {:?} ",part);
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

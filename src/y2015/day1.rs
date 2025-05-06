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

fn cl(acc: isize, c: char) -> isize {
    if c == '(' { acc + 1 } else { acc - 1 }
}
#[allow(unused_variables)]
pub fn part1(content: String) -> Result<(), Report> {
    for l in content.lines() {
        println!(
            "{}",
            l.chars()
                .fold(0,cl) 
        )
    }
    Ok(())
}
#[allow(unused_variables)]
pub fn part2(content: String) -> Result<(), Report> {
    for l in content.lines() {
        println!(
            "{:?}",
            l.chars().fold((0, 0), |acc, c| if acc.0 == -1 {
                (acc.0, acc.1)
            } else if c == '(' {
                (acc.0 + 1, acc.1 + 1)
            } else {
                (acc.0 - 1, acc.1 + 1)
            })
        )
    }
    Ok(())
}

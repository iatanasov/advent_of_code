#![allow(dead_code)]
#![allow(unused_variables)]
use aoc_2022::utils::Part;
use color_eyre::Report;
use tracing::info;

pub fn execute(content: String, part: Part) -> Result<(), Report> {
    info!("Running {:?} ", part);
    match part {
        Part::One => part1(content),
        Part::Two => part2(content),
    }
}

pub fn part1(content: String) -> Result<(), Report> {
    let mut count = 0;
    for l in content.lines() {
        let report: Vec<isize> = l.split(' ').map(|s| s.parse::<isize>().unwrap()).collect();
        if is_good(&report) {
            count += 1;
        }
    }
    println!("Answer {} ", count);
    Ok(())
}
pub fn part2(content: String) -> Result<(), Report> {
    let mut count = 0;
    for l in content.lines() {
        let report: Vec<isize> = l.split(' ').map(|s| s.parse::<isize>().unwrap()).collect();
        if is_good(&report) {
            count += 1;
        } else {
            for i in 0..report.len() {
                let mut new_r = report.clone();
                new_r.remove(i);
                if is_good(&new_r) {
                    count += 1;
                    break;
                }
            }
        }
    }
    println!("Answer {} ", count);
    Ok(())
}

pub fn is_good(report: &Vec<isize>) -> bool {
    let mut sign_num = 0;
    for (i, curr) in report.iter().enumerate() {
        let next = report.get(i + 1);
        if next.is_none() {
            println!("good {:?}", report);
            return true;
        }
        let val = curr - *next.unwrap();
        if val.abs() > 3 || val == 0 || (sign_num != 0 && sign_num != val.signum()) {
            return false;
        }
        sign_num = val.signum();
    }
    false
}

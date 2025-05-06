use aoc::utils::Part;
use color_eyre::Report;
use tracing::info;

pub fn execute(content: String, part: Part) -> Result<(), Report> {
    match part {
        Part::One => part1(content),
        Part::Two => part2(content),
    }
}

pub fn part1(content: String) -> Result<(), Report> {
    let mut count = 0;
    let frame: &mut Vec<char> = &mut vec![];
    for l in content.lines() {
        for c in l.chars() {
            frame.insert(0, c);
            count += 1;
            if frame.len() > 4 {
                frame.pop();
            }
            if frame.len() < 4 {
                continue;
            }
            let mut uniq = true;
            info!("frame {:?} ", frame);
            for i in 0..frame.len() {
                for j in i + 1..frame.len() {
                    info!(" {} {} ", i, j);
                    if frame[i] == frame[j] {
                        uniq = false;
                    }
                }
            }
            if uniq == true {
                info!("Start point {} ", count);
                break;
            }
        }
    }
    Ok(())
}
pub fn part2(content: String) -> Result<(), Report> {
    let mut count = 0;
    let frame: &mut Vec<char> = &mut vec![];
    for l in content.lines() {
        for c in l.chars() {
            frame.insert(0, c);
            count += 1;
            if frame.len() > 14 {
                frame.pop();
            }
            if frame.len() < 14 {
                continue;
            }
            let mut uniq = true;
            for i in 0..frame.len() {
                for j in i + 1..frame.len() {
                    if frame[i] == frame[j] {
                        uniq = false;
                    }
                }
            }
            if uniq == true {
                info!("Start point {} ", count);
                break;
            }
        }
    }
    Ok(())
}

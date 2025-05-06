#![allow(dead_code)]
#![allow(unused_variables)]
use aoc::utils::Part;
use color_eyre::Report;
use tracing::{debug, info};

pub fn execute(content: String, part: Part) -> Result<(), Report> {
    info!("Running {:?} ", part);
    match part {
        Part::One => part1(content),
        Part::Two => part2(content),
    }
}

fn is_visible(top: i32, left: i32, mid: i32, right: i32, bottom: i32) -> bool {
    let mut visible = false;
    if mid > top || mid > left || mid > right || mid > bottom {
        visible = true;
    }
    debug!(
        "{} {} [{}] {} {} res: {}",
        top, left, mid, right, bottom, visible
    );
    return visible;
}

#[allow(unused_variables)]
pub fn part1(content: String) -> Result<(), Report> {
    let mut count: usize = 0;
    let mut lines: Vec<Vec<i32>> = vec![];
    for l in content.lines() {
        lines.push(
            l.chars()
                .into_iter()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect(),
        );
    }
    debug!("{:?}", lines);
    let line_len = lines[0].len();
    info!("line length: {}", line_len);
    let mut max_top_line: Vec<i32> = vec![-1; line_len];
    let mut max_left: Vec<i32> = vec![-1; lines.len()];
    for i in 0..lines.len() {
        let current_line = &lines[i];
        for j in 0..line_len {
            let mid = current_line[j];
            let left = max_left[i];
            let top = max_top_line[j];
            let mut right = -1;
            if j < line_len - 1 {
                for k in j + 1..line_len {
                    if current_line[k] >= right {
                        right = current_line[k];
                    }
                }
            }
            let mut bottom = -1;
            if i < lines.len() - 1 {
                for k in i + 1..lines.len() {
                    if lines[k][j] >= bottom {
                        bottom = lines[k][j];
                    }
                }
            }
            if is_visible(top, left, mid, right, bottom) {
                count += 1;
            }
            if mid >= max_left[i] {
                max_left[i] = mid;
            }
            if mid >= max_top_line[j] {
                max_top_line[j] = mid;
            }
        }
    }
    info!("Max left {:?}", max_left);
    info!("Result: {} ", count);
    Ok(())
}
pub fn part2(content: String) -> Result<(), Report> {
    let mut max_score: usize = 0;
    let mut index = (0, 0);
    let mut lines: Vec<Vec<i32>> = vec![];
    for l in content.lines() {
        lines.push(
            l.chars()
                .into_iter()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect(),
        );
    }
    debug!("{:?}", lines);
    let line_len = lines[0].len();
    let max_lines = lines.len();
    info!("line length: {}", line_len);
    for i in 0..max_lines {
        let current_line = &lines[i];
        for j in 0..line_len {
            let mid = current_line[j];
            let mut right_score = 0;
            for k in j + 1..line_len {
                if current_line[k] >= mid {
                    right_score = k - j;
                    break;
                }
            }
            if right_score == 0 {
                right_score = max_lines - (j + 1);
            }
            let mut left_score = j;
            for k in (0..j).rev() {
                if current_line[k] >= mid {
                    left_score = j - k;
                    break;
                }
            }
            let mut bottom_score = 0;
            for k in i + 1..max_lines {
                if lines[k][j] >= mid {
                    bottom_score = k - i;
                    break;
                }
            }
            if bottom_score == 0 {
                bottom_score = max_lines - (i + 1);
            }
            let mut top_score = i;
            for k in (0..i).rev() {
                if lines[k][j] >= mid {
                    top_score = i - k;
                    break;
                }
            }
            debug!(
                "value: {} , r:{} l:{} b:{} t:{} ",
                lines[i][j], right_score, left_score, bottom_score, top_score
            );
            let score = right_score * left_score * bottom_score * top_score;
            if score > max_score {
                debug!(
                    "max score: {} value: {} , r:{} l:{} b:{} t:{} ",
                    score, lines[i][j], right_score, left_score, bottom_score, top_score
                );
                index = (i, j);
                max_score = score
            }
        }
    }
    info!(
        "Index: {} {} Item : {}",
        index.0, index.1, lines[index.0][index.1]
    );
    info!("Result: {} ", max_score);
    Ok(())
}

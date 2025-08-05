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
    let mut score = 0;
    for l in content.lines() {
        let v: Vec<&str> = l.split(' ').collect();
        let val = round(v[0], v[1]);
        info!("{} {} {} ", v[0], v[1], val);
        score += val;
    }
    info!("Result {}", score);
    Ok(())
}

fn round(p1: &str, p2: &str) -> usize {
    match p2 {
        "X" => match p1 {
            "A" => 1 + 3,
            "B" => 1,
            "C" => 1 + 6,
            _ => 0,
        },
        "Y" => {
            2 + match p1 {
                "A" => 6,
                "B" => 3,
                "C" => 0,
                _ => 0,
            }
        }
        "Z" => {
            3 + match p1 {
                "A" => 0,
                "B" => 6,
                "C" => 3,
                _ => 0,
            }
        }
        _ => 0,
    }
}

fn round2(p1: &str, p2: &str) -> usize {
    match p2 {
        "X" => {
            match p1 {
                "A" => 3,
                "B" => 1,
                "C" => 2,
                _ => 0,
            }
        }
        "Y" => {
            3 + match p1 {
                "A" => 1,
                "B" => 2,
                "C" => 3,
                _ => 0,
            }
        }
        "Z" => {
            6 + match p1 {
                "A" => 2,
                "B" => 3,
                "C" => 1,
                _ => 0,
            }
        }
        _ => 0,
    }
}

pub fn part2(content: String) -> Result<(), Report> {
    let mut score = 0;
    for l in content.lines() {
        let v: Vec<&str> = l.split(' ').collect();
        let val = round2(v[0], v[1]);
        info!("{} {} {} ", v[0], v[1], val);
        score += val;
    }
    info!("Result {}", score);
    Ok(())
}

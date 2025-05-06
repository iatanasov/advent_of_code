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
    let mut max_cal = 0;
    let mut current_cal = 0;
    for l in content.lines() {
        if l == "" {
            if current_cal > max_cal {
                max_cal = current_cal
            }
            current_cal = 0;
        } else {
            let val = l.parse::<usize>().unwrap();
            current_cal += val;
        }
    }
    info!("Result {}", max_cal);
    Ok(())
}

pub fn part2(content: String) -> Result<(), Report> {
    let mut top: Vec<usize> = vec![0, 0, 0];
    let mut current_cal = 0;
    for l in content.lines() {
        if l == "" {
            for m in &mut top {
                if current_cal > *m {
                    let current = *m;
                    *m = current_cal;
                    current_cal = current;
                }
            }
            current_cal = 0;
        } else {
            let val = l.parse::<usize>().unwrap();
            current_cal += val;
        }
    }
    info!("Result: {}", top.iter().sum::<usize>());
    Ok(())
}

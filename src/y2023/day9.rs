#![allow(dead_code)]
#![allow(unused_variables)]
use aoc::utils::{Part, str_to_isize};
use color_eyre::Report;
use tracing::info;

pub fn execute(content: String, part: Part) -> Result<(), Report> {
    info!("Day 9: Mirage Maintenance {:?} ", part);
    match part {
        Part::One => part1(content),
        Part::Two => part2(content),
    }
}

#[allow(unused_variables)]
pub fn part1(content: String) -> Result<(), Report> {
    let mut sum = 0;
    let mut history: Vec<isize> = vec![];
    for l in content.lines() {
        let mut grid: Vec<Vec<isize>> = vec![];
        let mut data = l.split(' ').map(str_to_isize).collect::<Vec<isize>>();
        grid.push(data);
        let mut brk = false;
        loop {
            if brk {
                break;
            }
            data = get_pred_vec(grid.last().unwrap());
            grid.push(data.clone());
            brk = !data.iter().any(|&x| x != 0);
        }
        let glen = grid.len();
        for n in (0..grid.len()).rev() {
            let last = grid[n].last().unwrap();
            let mut r = 0;
            if n < glen - 1 {
                r = grid[n + 1].last().unwrap() + *last
            };
            grid[n].push(r);
        }
        println!("Step");
        for g in &grid {
            println!("{:?}", g);
        }
        history.push(*grid[0].last().unwrap());
        sum += grid[0].last().unwrap();
        println!("Temp {}", sum);
    }

    println!("{:?}", history);
    println!("{}", sum);
    Ok(())
}
fn get_pred_vec(data: &[isize]) -> Vec<isize> {
    let mut ret_v: Vec<isize> = vec![];
    for wind in data.windows(2) {
        //println!("{:?}", wind);
        ret_v.push(wind[1] - wind[0]);
    }
    ret_v
}
#[allow(unused_variables)]
pub fn part2(content: String) -> Result<(), Report> {
    let mut sum = 0;
    let mut history: Vec<isize> = vec![];
    for l in content.lines() {
        let mut grid: Vec<Vec<isize>> = vec![];
        let mut data = l.split(' ').map(str_to_isize).collect::<Vec<isize>>();
        data.reverse();
        grid.push(data);
        let mut brk = false;
        loop {
            if brk {
                break;
            }
            data = get_pred_vec(grid.last().unwrap());
            grid.push(data.clone());
            brk = !data.iter().any(|&x| x != 0);
        }
        let glen = grid.len();
        for n in (0..grid.len()).rev() {
            let last = grid[n].last().unwrap();
            let mut r = 0;
            if n < glen - 1 {
                r = grid[n + 1].last().unwrap() + *last
            };
            grid[n].push(r);
        }
        println!("Step");
        for g in &grid {
            println!("{:?}", g);
        }
        history.push(*grid[0].last().unwrap());
        sum += grid[0].last().unwrap();
        println!("Temp {}", sum);
    }

    println!("{:?}", history);
    println!("{}", sum);
    Ok(())
}

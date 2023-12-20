#![allow(dead_code)]
#![allow(unused_variables)]
use std::{collections::HashMap};

use tracing::info;
use color_eyre::Report;
use aoc_2022::utils::{Part, to_digits};


pub fn execute(content: String, part: Part) -> Result<(), Report> {
    info!("Running {:?} ",part);
    match part {
        Part::One => part1(content),
        Part::Two => part2(content),
    }
}

fn is_symbol(s: char) -> bool {
    if s.is_digit(10) || s == '.' {
        return false;
    }
    return true
}

fn get_range(n: usize, max: usize) -> Vec<usize> {
    let mut r  = vec![n];
    if n > 0 {
        r.insert(0, n -1 );
    }
    if n + 1 < max {
        r.push(n+1);
    }
    return r
}
    
fn adj(engine: &Vec<Vec<char>>, i: usize, j: usize ) -> bool {
    for n in get_range(i,engine.len()) {
        for k in get_range(j, engine[i].len()) {
            if is_symbol(engine[n][k]) { return true; }
        }
    }
    return false
}

#[allow(unused_variables)]
pub fn part1(content: String) -> Result<(), Report> {
    let mut sum = 0;
    let mut engine: Vec<Vec<char>> = vec![];
    for l in content.lines() {
        engine.push(l.chars().collect());
    }
    let mut num: Vec<char> = vec![];
    let mut is_adj = false;
    for i in 0..engine.len() {
        if is_adj && num.len() > 0 {
            sum = sum + format!(
                "{}",
                num.iter().collect::<String>()
                ).parse::<usize>().unwrap();
        }
        is_adj = false;
        num = vec![];
        for j in 0..engine[i].len() {
            let s = engine[i][j];
            if s.is_digit(10) {
                num.push(s); 
                if is_adj == false {
                    is_adj = adj(&engine,i,j);
                }
            } else {
                if is_adj && num.len() > 0 {
                    sum = sum + format!(
                        "{}",
                        num.iter().collect::<String>()
                        ).parse::<usize>().unwrap();
                }
                is_adj = false;
                num = vec![];
            }
        }
    }
    println!("sum: {}", sum);
    Ok(())
}
fn is_gear(s: char) -> bool {
    if s == '*' {
        return true
    }
    return false
}
fn gear_adj(engine: &Vec<Vec<char>>, i: usize, j: usize ) -> Option<(usize, usize)> {
    for n in get_range(i,engine.len()) {
        for k in get_range(j, engine[i].len()) {
            if is_gear(engine[n][k]) { return Some((n,k)) }
        }
    }
    return None
}
#[allow(unused_variables)]
pub fn part2(content: String) -> Result<(), Report> {
    let mut product = 0;
    let mut engine: Vec<Vec<char>> = vec![];
    let mut gears: HashMap<(usize, usize),Vec<usize>> = HashMap::new();
    for l in content.lines() {
        engine.push(l.chars().collect());
    }
    let mut num: Vec<char> = vec![];
    let mut is_adj: Option<(usize,usize)> = None;
    for i in 0..engine.len() {
        if is_adj.is_some() && num.len() > 0 {
            let s = is_adj.unwrap();
            match gears.get_mut(&(s.0,s.1)) {
                Some(arr) => {
                    arr.push(to_digits(&num))
                }
                None => {
                    gears.insert((s.0,s.1), vec![to_digits(&num)]);
                    ()
                }
            }
        }
        is_adj  = None;
        num = vec![];
        for j in 0..engine[i].len() {
            let s = engine[i][j];
            if s.is_digit(10) {
                num.push(s); 
                if is_adj.is_none(){
                    is_adj = gear_adj(&engine,i,j);
                }
            } else {
                if is_adj.is_some() && num.len() > 0 {
                    let s = is_adj.unwrap();
                    match gears.get_mut(&(s.0,s.1)) {
                        Some(arr) => {
                            arr.push(to_digits(&num))
                        }
                        None => {
                            gears.insert((s.0,s.1), vec![to_digits(&num)]);
                            ()
                        }
                    }
                }
                is_adj = None;
                num = vec![];
            }
        }
    }
    for (k,v) in gears.iter(){ 
        if v.len() > 1 {
            product = product + v[0]*v[1];
        }
    }
    println!("sum: {}",product);
    Ok(())
}

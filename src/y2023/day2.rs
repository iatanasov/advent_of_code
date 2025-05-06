#![allow(dead_code)]
#![allow(unused_variables)]
use aoc::utils::Part;
use color_eyre::Report;
use regex::Regex;
use std::collections::HashMap;
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
    let limits: HashMap<&str, usize> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let mut sum = 0;
    let game_re = Regex::new(r"Game\s(\d+):(.*)").unwrap();
    for l in content.lines() {
        let cap = game_re.captures(l).unwrap();
        let game_n = cap[1].parse::<usize>().unwrap();
        let tokens: Vec<&str> = cap[2].split([';', ',']).collect();
        let r: Vec<_> = tokens
            .iter()
            .filter(|t| {
                let v = t.trim_start().split(' ').collect::<Vec<&str>>();
                let n = v[0].parse::<usize>().unwrap();
                let k = v[1];
                return n > *limits.get(k).unwrap();
            })
            .collect();
        if r.len() == 0 {
            sum = sum + game_n;
        }
        info!("{:?}", r);
    }
    println!("{}", sum);
    Ok(())
}
pub struct GameSet {
    red: usize,
    green: usize,
    blue: usize,
}

impl GameSet {
    fn new() -> Self {
        GameSet {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
    fn set_max(&mut self, color: &str, value: usize) {
        match color {
            "red" => {
                if self.red < value {
                    self.red = value
                }
            }
            "green" => {
                if self.green < value {
                    self.green = value
                }
            }
            "blue" => {
                if self.blue < value {
                    self.blue = value
                }
            }
            _ => panic!("Bad color {}", color),
        }
    }
    fn power(&self) -> usize {
        return self.red * self.green * self.blue;
    }
}

#[allow(unused_variables)]
pub fn part2(content: String) -> Result<(), Report> {
    let mut sum = 0;
    let game_re = Regex::new(r"Game\s(\d+):(.*)").unwrap();
    for l in content.lines() {
        let cap = game_re.captures(l).unwrap();
        let tokens: Vec<&str> = cap[2].split([';', ',']).collect();
        let mut game_set = GameSet::new();
        for t in tokens {
            let v = t.trim_start().split(' ').collect::<Vec<&str>>();
            let n = v[0].parse::<usize>().unwrap();
            game_set.set_max(v[1], n);
        }
        sum = sum + game_set.power();
    }
    println!("{}", sum);
    Ok(())
}

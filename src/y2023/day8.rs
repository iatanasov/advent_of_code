#![allow(dead_code)]
#![allow(unused_variables)]
use aoc::utils::Part;
use color_eyre::Report;
use regex::Regex;
use std::{collections::HashMap, sync::OnceLock};
use tracing::{info, trace};

pub fn map_regex() -> &'static Regex {
    static MAP_REGEX: OnceLock<Regex> = OnceLock::new();
    MAP_REGEX.get_or_init(|| {
        let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)");
        re.unwrap()
    })
}

pub fn execute(content: String, part: Part) -> Result<(), Report> {
    info!("Running {:?} ", part);
    match part {
        Part::One => part1(content),
        Part::Two => part2(content),
    }
}

#[allow(unused_variables)]
pub fn part1(content: String) -> Result<(), Report> {
    let mut lines = content.lines();
    let directions: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut order: Vec<&str> = vec![];
    for l in lines {
        let cap = map_regex().captures(l).unwrap();
        order.push(cap.get(1).unwrap().as_str());
        map.insert(
            cap.get(1).unwrap().as_str(),
            (cap.get(2).unwrap().as_str(), cap.get(3).unwrap().as_str()),
        );
    }
    let mut start: &str = "AAA";
    let end: &str = "ZZZ";
    let mut steps = 0;
    loop {
        info!("Compare {} {}", start, end);
        if start == end {
            break;
        }
        for d in &directions {
            let node = map.get(&start).unwrap();
            trace!("Compare root {} node {} {} {}", &start, node.0, node.1, d);
            if *d == 'L' {
                start = node.0;
            } else {
                start = node.1;
            }
            steps += 1;
            if start == end {
                break;
            }
        }
        info!("Loop completed steps {}", steps);
    }
    println!("{}", steps);
    Ok(())
}

fn find_node() {}
#[allow(unused_variables)]
pub fn part2(content: String) -> Result<(), Report> {
    let mut start: Vec<&str> = vec![];
    let mut cycle: Vec<usize> = vec![];
    let mut lines = content.lines();
    let directions: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    for l in lines {
        let cap = map_regex().captures(l).unwrap();
        let o = cap.get(1).unwrap().as_str();
        if o.ends_with("A") {
            start.push(o);
            cycle.push(0);
        }

        map.insert(
            cap.get(1).unwrap().as_str(),
            (cap.get(2).unwrap().as_str(), cap.get(3).unwrap().as_str()),
        );
    }
    let mut steps = 0;
    let mut has_z = 0;
    let total_c = &start.len();
    let mut counter = 1;
    loop {
        for d in &directions {
            for (i, s) in start.iter_mut().enumerate() {
                let node = map.get(s).unwrap();
                trace!("Compare root {} node {} {} {}", &s, node.0, node.1, d);
                if *d == 'L' {
                    *s = node.0;
                } else {
                    *s = node.1;
                }
                if s.ends_with("Z") {
                    has_z += 1;
                    if cycle[i] == 0 {
                        cycle[i] = counter;
                    }
                }
            }
            counter += 1;
            steps += 1;
            if has_z >= 1 {
                let total = &cycle.iter().filter(|x| **x == 0).count();
                if *total == 0 {
                    println!("{:?}", &cycle);
                    println!("{}", lcm(cycle.as_mut()));
                    return Ok(());
                }
            }
            has_z = 0;
            //info!("Loop completed steps {}", steps);
        }
    }
}

fn lcm(nums: &mut Vec<usize>) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums.remove(0);
    let b = lcm(nums);

    a * b / gcd_two(a, b)
}

fn gcd(nums: &mut Vec<usize>) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums.remove(0);
    let b = gcd(nums);
    gcd_two(a, b)
}

fn gcd_two(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_two(b, a % b)
}

#![allow(dead_code)]
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

pub fn part1(content: String) -> Result<(), Report> {
    let mut list1: Vec<usize> = vec![];
    let mut list2: Vec<usize> = vec![];
    for l in content.lines() {
        let res: Vec<&str> = l.split(' ').collect();
        list1.push(res[0].parse::<usize>().unwrap());
        list2.push(res[3].parse::<usize>().unwrap());
    }
    list1.sort();
    list2.sort();
    let mut total = 0;
    println!("{:?}", list1);
    for (i, a) in list1.iter().enumerate() {
        println!("{i}");
        let b = list2[i];
        println!("a {} b {}", a, b);
        total += a.abs_diff(b);
    }
    println!("total {total}");
    Ok(())
}
#[allow(unused_variables)]
pub fn part2(content: String) -> Result<(), Report> {
    let mut list1: Vec<usize> = vec![];
    let mut list2: Vec<usize> = vec![];
    for l in content.lines() {
        let res: Vec<&str> = l.split(' ').collect();
        list1.push(res[0].parse::<usize>().unwrap());
        list2.push(res[3].parse::<usize>().unwrap());
    }
    list1.sort();
    list2.sort();
    let mut total = 0;
    for n in &list1 {
        let mut s = 0;
        for k in &list2 {
            //println!("{n} {k}");
            match n.cmp(k) {
                std::cmp::Ordering::Less => break,
                std::cmp::Ordering::Equal => s += 1,
                std::cmp::Ordering::Greater => (),
            }
        }
        total += n * s;
    }
    println!("total {total}");
    Ok(())
}

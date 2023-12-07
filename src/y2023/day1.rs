#![allow(dead_code)]
#![allow(unused_variables)]
use std::collections::HashMap;
use tracing::info;
use color_eyre::Report;
use aoc_2022::utils::Part;


pub fn execute(content: String, part: Part) -> Result<(), Report> {
    info!("Running {:?} ",part);
    match part {
        Part::One => part1(content),
        Part::Two => part2(content),
    }
}

#[allow(unused_variables)]
pub fn part1(content: String) -> Result<(), Report> {
    let mut sum = 0;
    for l in content.lines() {
        let digits: Vec<String> = l.chars()
            .filter(|x| x.is_numeric())
            .collect::<Vec<char>>().into_iter()
            .map(|c| c.to_string()).collect();
        sum = sum + format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
            .parse::<u32>().unwrap();
    }
    println!("{}",sum);
    Ok(())
}
#[allow(unused_variables)]
pub fn part2(content: String) -> Result<(), Report> {
    let dmap: HashMap<&str, usize>  = HashMap::from([
        ("one", 1),
        ("two",2),
        ("three",3),
        ("four",4),
        ("five", 5),
        ("six",6),
        ("seven",7),
        ("eight",8),
        ("nine",9),
    ]);
    let mut sum = 0 as usize;
    for l in content.lines() {
        let mut numbers: HashMap<usize, usize> = HashMap::new();
        for k in dmap.keys() {
            let foo: Vec<(usize, &str)> = l.match_indices(k).collect::<Vec<(usize,&str)>>();
            foo.iter().for_each(|(i,a)| { 
                numbers.insert(*i, *dmap.get(a).unwrap());
            });
        }
        l.chars()
            .enumerate()
            .for_each(|(i,x)| {
                if x.is_numeric() {
                    numbers.insert(i, x.to_digit(10).unwrap() as usize);
                }
            });
        let mut skeys = numbers.clone().into_keys().collect::<Vec<usize>>();
        skeys.sort();
        let dd = format!("{}{}",
            numbers.get(skeys.first().unwrap()).unwrap(),
            numbers.get(skeys.last().unwrap()).unwrap()
        ).parse::<usize>().unwrap();

        sum = sum + dd;
    }
    println!("{}",sum);
    Ok(())
}

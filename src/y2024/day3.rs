use aoc::utils::Part;
use color_eyre::Report;
use regex::Regex;
use std::sync::OnceLock;
use tracing::info;

fn memory_regex() -> &'static Regex {
    static MEMORY_REGEX: OnceLock<Regex> = OnceLock::new();
    MEMORY_REGEX.get_or_init(|| {
        let re = Regex::new(r"mul\((\d+),(\d+)\)");
        re.unwrap()
    })
}
fn memory_regex2() -> &'static Regex {
    static MEMORY_REGEX: OnceLock<Regex> = OnceLock::new();
    MEMORY_REGEX.get_or_init(|| {
        let re = Regex::new(r"(mul\(\d+,\d+\)|do\(\)|don't\(\))");
        re.unwrap()
    })
}
fn num_regex() -> &'static Regex {
    static MEMORY_REGEX: OnceLock<Regex> = OnceLock::new();
    MEMORY_REGEX.get_or_init(|| {
        let re = Regex::new(r"mul\((\d+),(\d+)\)");
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
    let mut sum = 0;
    for l in content.lines() {
        for (_, [seed, seed2]) in memory_regex().captures_iter(l).map(|c| {
            println!("{:?}", c);
            c.extract()
        }) {
            sum += seed.parse::<usize>().unwrap() * seed2.parse::<usize>().unwrap();
        }
    }
    println!("sum {sum}");
    Ok(())
}
#[allow(unused_variables)]
pub fn part2(content: String) -> Result<(), Report> {
    let mut sum = 0;
    let mut doit = true;
    for l in content.lines() {
        for (_, [got]) in memory_regex2().captures_iter(l).map(|c| {
            println!("{:?}", c);
            c.extract()
        }) {
            println!("{got}");
            match got {
                "do()" => doit = true,
                "don't()" => doit = false,
                _ => {
                    if doit {
                        let caps = num_regex().captures(got).unwrap();
                        println!("booo {:?}", caps);
                        sum +=
                            caps[1].parse::<usize>().unwrap() * caps[2].parse::<usize>().unwrap();
                    }
                } //sum += seed.parse::<usize>().unwrap() * seed2.parse::<usize>().unwrap();
            }
        }
    }
    println!("sum {sum}");
    Ok(())
}

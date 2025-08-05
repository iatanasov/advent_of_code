#![allow(dead_code)]
#![allow(unused_variables)]
use std::sync::OnceLock;

use aoc::utils::{Part, str_to_digits};
use color_eyre::Report;
use regex::Regex;
use tracing::info;

fn seeds_regex() -> &'static Regex {
    static SEEDS_REGEX: OnceLock<Regex> = OnceLock::new();
    SEEDS_REGEX.get_or_init(|| {
        let re = Regex::new(r"([\d]+)");
        re.unwrap()
    })
}
fn maps_regex() -> &'static Regex {
    static MAPS_REGEX: OnceLock<Regex> = OnceLock::new();
    MAPS_REGEX.get_or_init(|| {
        let re = Regex::new(r"([\d]+)\s+([\d]+)\s+([\d]+)");
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
    let mut seeds: Vec<Vec<usize>> = Vec::new();
    for (_, [seed]) in seeds_regex()
        .captures_iter(lines.next().unwrap())
        .map(|c| c.extract())
    {
        seeds.push(vec![str_to_digits(seed)]);
    }
    lines.next();
    let mut level = 0;
    let mut ranges: Vec<Vec<usize>> = Vec::new();
    loop {
        let res = lines.next();
        let l = res.unwrap_or("");

        if l.is_empty() {
            for seed in &mut seeds {
                let mut found = false;
                for r in &ranges {
                    if !found {
                        found = match src_to_dest(r[0], r[1], r[2], seed[level]) {
                            Some(d) => {
                                seed.push(d);
                                true
                            }
                            None => false,
                        };
                    }
                }
                if !found {
                    seed.push(seed[level]);
                }
            }
            level += 1;
            if res.is_none() {
                break;
            }
        } else if l.contains("map") {
            ranges = Vec::new();
        } else {
            let caps = maps_regex().captures(l).unwrap();
            ranges.push(vec![
                str_to_digits(caps.get(1).unwrap().as_str()),
                str_to_digits(caps.get(2).unwrap().as_str()),
                str_to_digits(caps.get(3).unwrap().as_str()),
            ]);
        }
    }
    let mut location = seeds.into_iter().map(|x| x[level]).collect::<Vec<usize>>();
    location.sort();
    println!("{}", location[0]);
    Ok(())
}
fn src_to_dest(dest: usize, src: usize, range: usize, val: usize) -> Option<usize> {
    //println!("Compare val {} >= src {} && val {} <={}", val, src ,val, src+range);
    if val >= src && val <= src + range {
        return Some((val - src) + dest);
    }
    None
}
fn dest_to_src(dest: usize, src: usize, offset: usize, val: usize) -> Option<usize> {
    //println!("Compare val {} >= src {} && val {} <={}", val, src ,val, src+offset);
    if val >= dest && val < dest + offset {
        return Some(val - dest + src);
    }
    None
}
fn exclusive_found_in_range(sorted_ranges: &[(usize, usize, usize)], val: usize) -> (usize, usize) {
    for (i, r) in sorted_ranges.iter().enumerate() {
        if val < r.0 {
            return (val, i);
        }
        let res = dest_to_src(r.0, r.1, r.2, val);
        if let Some(res1) = res {
            return (res.unwrap(), i);
        }
    }
    (val, 0)
}

#[allow(unused_variables)]
pub fn part2(content: String) -> Result<(), Report> {
    let mut ranges: Vec<Vec<(usize, usize, usize)>> = Vec::new();
    let mut range: Vec<(usize, usize, usize)> = Vec::new();

    let mut lines = content.lines();
    let mut pre_seeds: Vec<usize> = Vec::new();
    for (_, [seed]) in seeds_regex()
        .captures_iter(lines.next().unwrap())
        .map(|c| c.extract())
    {
        pre_seeds.push(str_to_digits(seed));
    }
    lines.next();
    loop {
        let res = lines.next();
        let l = res.unwrap_or("");
        if l.is_empty() {
            range.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            ranges.insert(0, range);
            range = Vec::new();
            if res.is_none() {
                break;
            }
        } else if l.contains("map") {
            println!("Mapping {}", l);
        } else {
            let caps = maps_regex().captures(l).unwrap();
            range.push((
                str_to_digits(caps.get(1).unwrap().as_str()),
                str_to_digits(caps.get(2).unwrap().as_str()),
                str_to_digits(caps.get(3).unwrap().as_str()),
            ));
        }
    }
    let mut it = pre_seeds.iter();
    loop {
        let i = it.next();
        let r = it.next();
        match i {
            Some(s) => {
                let e = r.unwrap();
                println!("i {:?} r {:?}", s, e);
                range.push((*s, *s, *e))
            }
            None => break,
        }
    }
    range.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    ranges.push(range);
    let mut n = 0;
    let mut val: usize;
    let mut found = false;
    loop {
        val = n;
        (0..8).for_each(|r| {
            if !found {
                if r == 7 {
                    for range in &ranges[r] {
                        if val >= range.0 && val <= range.0 + range.2 {
                            found = true;
                            break;
                        }
                    }
                } else {
                    let res = exclusive_found_in_range(&ranges[r], val);
                    val = res.0;
                }
            }
        });
        if found {
            break;
        }
        n += 1;
    }
    println!("{}", n);
    Ok(())
}

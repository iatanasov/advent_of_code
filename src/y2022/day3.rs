use aoc::utils::Part;
use color_eyre::Report;
use std::collections::HashMap;
use tracing::info;

pub fn execute(content: String, part: Part) -> Result<(), Report> {
    match part {
        Part::One => part1(content),
        Part::Two => part2(content),
    }
}
fn gen_priotiry() -> HashMap<char, usize> {
    let mut priority = (b'a'..=b'z').map(char::from).collect::<Vec<_>>();
    priority.append(&mut (b'A'..=b'Z').map(char::from).collect::<Vec<_>>());
    let mut pmap: HashMap<char, usize> = HashMap::new();
    for (i, p) in priority.into_iter().enumerate() {
        pmap.insert(p, i + 1);
    }

    pmap
}

pub fn part2(content: String) -> Result<(), Report> {
    let priority = gen_priotiry();
    let mut sum = 0;
    let mut uniq: Vec<char> = vec![];
    let mut group_n = 0;
    let mut group_badges: HashMap<char, usize> = HashMap::new();
    for l in content.lines() {
        group_n += 1;
        for i in l.chars() {
            if uniq.contains(&i) {
                continue;
            }
            uniq.push(i);
            let val = group_badges.get(&i);
            match val {
                Some(&n) => group_badges.insert(i.to_owned(), n + 1),
                None => group_badges.insert(i, 1),
            };
        }
        uniq = vec![];
        if group_n == 3 {
            for (ch, n) in group_badges.iter() {
                if *n == 3 {
                    info!(
                        "Found {} n {} priority {}",
                        &ch,
                        &n,
                        priority.get(ch).unwrap()
                    );
                    sum += priority.get(ch).unwrap();
                    break;
                }
            }
            group_n = 0;
            group_badges = HashMap::new();
        }
    }
    info!("Result: {}", sum);
    Ok(())
}

pub fn part1(content: String) -> Result<(), Report> {
    let priority = gen_priotiry();
    let mut wrong_items: Vec<char> = vec![];
    for l in content.lines() {
        let side_one = &l[0..l.len() / 2];
        let side_two = &l[l.len() / 2..];
        for i in side_one.chars() {
            let mut found = false;
            for j in side_two.chars() {
                if i == j {
                    wrong_items.push(i);
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
    }
    let mut sum = 0;
    for w in wrong_items {
        sum += priority.get(&w).unwrap();
    }
    info!("Result: {}", sum);
    Ok(())
}

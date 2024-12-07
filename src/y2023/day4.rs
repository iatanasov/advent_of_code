#![allow(dead_code)]
#![allow(unused_variables)]
use tracing::info;
use std::sync::OnceLock;
use color_eyre::Report;
use aoc_2022::utils::Part;
use regex::Regex;


fn card_regex() -> &'static Regex {
    static CARD_REGEX: OnceLock<Regex> = OnceLock::new();
    CARD_REGEX.get_or_init(|| {
        let re = Regex::new(r"Card\s+(\d+):([\s\d]+)\|([\s\d]+)");
            re.unwrap()
    })
}
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
        println!("{}",l);
        let cap = card_regex().captures(l).unwrap();
        let winning = &cap[2].trim_start().split(' ').into_iter()
            .filter(|&x| x.len() > 0)
            .map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let card = &cap[3].trim_start().split(' ').into_iter()
            .filter(|&x| x.len() > 0)
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let mut m_c = 0;
        for w in winning {
            for c in card {
                if w == c {
                    m_c += 1;
                }
            }
        }
        if m_c > 0 {
            sum = sum + i32::pow(2,m_c-1);
        }
    }
    println!("sum {}", sum); 
    Ok(())
}
#[derive(Debug, Clone)]
struct Card {
    n: usize,
    copies: usize,
    winning: usize,
    numbers: Vec<usize> 
}

impl Card {
    pub fn inc(&mut self, x: &usize) {
        self.copies = self.copies + 1*x;
    }
    pub fn copies(&self) -> usize {
        self.copies
    }
}
#[allow(unused_variables)]
pub fn part2(content: String) -> Result<(), Report> {
    let mut cards: Vec<Card> = vec![];
    for l in content.lines() {
        let cap = card_regex().captures(l).unwrap();
        let card_n = cap[1].parse::<usize>().unwrap();
        let win = &cap[2].trim_start().split(' ').into_iter()
            .filter(|&x| x.len() > 0)
            .map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let nums = &cap[3].trim_start().split(' ').into_iter()
            .filter(|&x| x.len() > 0)
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let mut m_c = 0;
        for w in win  {
            for c in nums {
                if w == c {
                    m_c += 1;
                }
            }
        }
        cards.push(Card{ n: card_n, copies: 1, winning: m_c, numbers: nums.to_vec() } );
    }
    let cards_len: usize = cards.len();
    for k in 0..cards_len {
        let r = cards[k].copies();
        let l = cards[k].winning;
        let j = k+1;
        for i in j..j+l {
            if i < cards_len  {
                let cn = &mut cards[i];
                cn.inc(&r);
            }
        }
    }
    let sum: usize = cards.into_iter().map(|c| c.copies()).collect::<Vec<usize>>().iter().sum();
    println!("sum {}", sum); 
    Ok(())
}

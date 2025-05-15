use std::collections::HashMap;

use aoc::year::DayParts;
use color_eyre::Report;

pub struct Day1 {
    pub content: String,
}
fn cl(acc: isize, c: char) -> isize {
    if c == '(' { acc + 1 } else { acc - 1 }
}
impl DayParts for Day1 {
    #[allow(unused_variables)]
    fn part1(&self) -> Result<(), Report> {
        for l in self.content.lines() {
            println!("{}", l.chars().fold(0, cl))
        }
        Ok(())
    }
    #[allow(unused_variables)]
    fn part2(&self) -> Result<(), Report> {
        for l in self.content.lines() {
            println!(
                "{:?}",
                l.chars().fold((0, 0), |acc, c| if acc.0 == -1 {
                    (acc.0, acc.1)
                } else if c == '(' {
                    (acc.0 + 1, acc.1 + 1)
                } else {
                    (acc.0 - 1, acc.1 + 1)
                })
            )
        }
        Ok(())
    }
}

pub struct Day2 {
    pub content: String,
}
impl DayParts for Day2 {
    #[allow(unused_variables)]
    fn part1(&self) -> Result<(), Report> {
        let sum: usize = self
            .content
            .lines()
            .map(|l| {
                let mut sides: Vec<usize> = l.split('x').map(|f| f.parse().unwrap()).collect();
                sides.sort();
                (3 * sides[0] * sides[1]) + (2 * sides[1] * sides[2]) + (2 * sides[0] * sides[2])
            })
            .sum();
        println!("{sum}");
        Ok(())
    }
    #[allow(unused_variables)]
    fn part2(&self) -> Result<(), Report> {
        let sum = 0;
        for l in self.content.lines() {
            let sides: Vec<usize> = l.split('x').map(|f| f.parse().unwrap()).collect();
        }
        println!("{sum}");
        Ok(())
    }
}

pub struct Day3 {
    pub content: String,
}
#[derive(Eq, Hash, PartialEq, Clone)]
struct House {
    x: isize,
    y: isize,
}

impl House {
    pub fn new(x: isize, y: isize) -> Self {
        House { x, y }
    }
    pub fn update(&mut self, direction: &char) {
        match direction {
            '^' => self.y += 1,
            '>' => self.x += 1,
            'v' => self.y -= 1,
            '<' => self.x -= 1,
            _ => panic!("Bad input {direction}"),
        }
    }
}

impl DayParts for Day3 {
    fn part1(&self) -> Result<(), Report> {
        let mut grid: HashMap<House, usize> = HashMap::new();
        let mut h = House::new(0, 0);
        grid.insert(h.clone(), 1);
        self.content.lines().next().unwrap().chars().for_each(|c| {
            h.update(&c);
            match grid.get_mut(&h) {
                Some(v) => *v += 1,
                None => _ = grid.insert(h.clone(), 1),
            }
        });
        println!("{}", grid.len());
        Ok(())
    }

    fn part2(&self) -> Result<(), Report> {
        let mut grid: HashMap<House, usize> = HashMap::new();
        let mut santa = House::new(0, 0);
        let mut robo_santa = House::new(0, 0);
        grid.insert(santa.clone(), 2);
        self.content
            .lines()
            .next()
            .unwrap()
            .chars()
            .enumerate()
            .for_each(|(i, c)| {
                let h = if i % 2 == 0 {
                    &mut santa
                } else {
                    &mut robo_santa
                };
                h.update(&c);
                match grid.get_mut(h) {
                    Some(v) => *v += 1,
                    None => _ = grid.insert(h.clone(), 1),
                }
            });
        println!("{}", grid.len());
        Ok(())
    }
}

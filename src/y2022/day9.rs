#![allow(dead_code)]
#![allow(unused_variables)]
use aoc::utils::Part;
use color_eyre::Report;
use std::collections::HashMap;
use tracing::info;

pub fn execute(content: String, part: Part) -> Result<(), Report> {
    info!("Running {:?} ", part);
    match part {
        Part::One => part1(content),
        Part::Two => part2(content),
    }
}
#[derive(Debug, PartialEq)]
enum Motion {
    R,
    L,
    U,
    D,
}

#[derive(Debug)]
struct Move {
    motion: Motion,
    steps: usize,
}

#[derive(Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        return Point { x, y };
    }

    pub fn head_move(&mut self, motion: &Motion, step: usize) {
        match motion {
            Motion::R => self.x = self.x + 1,
            Motion::L => self.x = self.x - 1,
            Motion::U => self.y = self.y + 1,
            Motion::D => self.y = self.y - 1,
        }
    }
    pub fn tail_move(&mut self, head: &Point) {
        if self.y == head.y {
            let d = head.x - self.x;
            match d {
                2 => self.x = self.x + 1,
                -2 => self.x = self.x - 1,
                _ => (),
            }
        } else if self.x == head.x {
            let d = head.y - self.y;
            match d {
                2 => self.y = self.y + 1,
                -2 => self.y = self.y - 1,
                _ => (),
            }
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

impl Move {
    pub fn new(line: &str) -> Move {
        let pair: Vec<&str> = line.split(' ').collect();
        let motion = match pair[0] {
            "R" => Motion::R,
            "L" => Motion::L,
            "U" => Motion::U,
            "D" => Motion::D,
            _ => panic!("Bad pair"),
        };
        let steps = pair[1].parse::<usize>().unwrap();
        return Move { motion, steps };
    }
}

pub fn part1(content: String) -> Result<(), Report> {
    let mut head = Point::new(0, 0);
    let mut tail = Point::new(0, 0);
    let coords: HashMap<Point, usize> = HashMap::new();
    for l in content.lines() {
        let m = Move::new(l);
        info!("{:?}", m);
        for s in 0..m.steps {
            if head == tail {
                head.head_move(&m.motion, 1);
            } else {
                head.head_move(&m.motion, 1);
                tail.tail_move(&head);
            }
        }
        info!("{:?}", tail);
    }
    Ok(())
}

#[allow(unused_variables)]
pub fn part2(content: String) -> Result<(), Report> {
    Ok(())
}

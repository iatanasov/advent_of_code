use std::{
    char,
    collections::{HashMap, HashSet},
};

use aoc::year::DayParts;
use color_eyre::Report;
use regex::Regex;

pub struct Day1 {
    pub content: String,
}
fn cl(acc: isize, c: char) -> isize {
    if c == '(' { acc + 1 } else { acc - 1 }
}
impl DayParts for Day1 {
    #[allow(unused_variables)]
    fn part1(&mut self) -> Result<(), Report> {
        for l in self.content.lines() {
            println!("{}", l.chars().fold(0, cl))
        }
        Ok(())
    }
    #[allow(unused_variables)]
    fn part2(&mut self) -> Result<(), Report> {
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
    fn part1(&mut self) -> Result<(), Report> {
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
    fn part2(&mut self) -> Result<(), Report> {
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
    fn part1(&mut self) -> Result<(), Report> {
        let mut grid: HashMap<House, usize> = HashMap::new();
        let mut h = House::new(0, 0);
        grid.insert(h.clone(), 1);
        self.content.lines().next().unwrap().chars().for_each(|c| {
            h.update(&c);
            grid.entry(h.clone()).and_modify(|v| *v += 1).or_insert(1);
        });
        println!("{}", grid.len());
        Ok(())
    }

    fn part2(&mut self) -> Result<(), Report> {
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
                grid.entry(h.clone()).and_modify(|v| *v += 1).or_insert(1);
            });
        println!("{}", grid.len());
        Ok(())
    }
}

pub struct Day4 {
    pub content: String,
}

impl DayParts for Day4 {
    fn part1(&mut self) -> Result<(), Report> {
        let secret = self.content.lines().next().unwrap();
        let mut i = 0;
        loop {
            let data = secret.to_owned() + i.to_string().as_str();
            let md5 = md5::compute(&data);
            if let [0, 0, 4, ..] = md5.0 {
                break;
            }
            i += 1;
        }
        println!("one {i}");
        Ok(())
    }

    fn part2(&mut self) -> Result<(), Report> {
        let secret = self.content.lines().next().unwrap();
        let mut i = 0;
        let l = "000000";
        loop {
            let data = format!("{secret}{i}");
            let mut s = format!("{:x}", md5::compute(&data));
            s.truncate(6);
            if s == l {
                break;
            }
            i += 1;
        }
        println!("two {i}");
        Ok(())
    }
}

pub struct Day5 {
    pub content: String,
}

impl DayParts for Day5 {
    fn part1(&mut self) -> Result<(), Report> {
        let mut sum = 0;
        for l in self.content.lines() {
            let mut vowels = 0;
            let mut doubles = false;
            let mut bad = false;
            let chars: Vec<char> = l.chars().collect();
            for i in 0..chars.len() {
                let s1 = chars.get(i).unwrap();
                let c2 = chars.get(i + 1);
                match s1 {
                    'a' | 'e' | 'i' | 'o' | 'u' => vowels += 1,
                    _ => (),
                }
                if let Some(s2) = c2 {
                    if s1 == s2 {
                        doubles = true;
                    } else if matches!(
                        (s1, s2),
                        (&'a', &'b') | (&'c', &'d') | (&'p', &'q') | (&'x', &'y')
                    ) {
                        bad = true;
                        break;
                    }
                }
            }
            if !bad && vowels > 2 && doubles {
                sum += 1;
            }
        }
        print!(" {sum} ");
        Ok(())
    }

    fn part2(&mut self) -> Result<(), Report> {
        let mut sum = 0;
        for l in self.content.lines() {
            let mut pair = false;
            let mut triplet = false;
            let chars: Vec<char> = l.chars().collect();
            let len = chars.len();
            for i in 0..len {
                if chars.get(i) == chars.get(i + 2) {
                    triplet = true;
                }
                for j in i + 2..(len - 1) {
                    if (chars.get(i), chars.get(i + 1)) == (chars.get(j), chars.get(j + 1)) {
                        pair = true
                    }
                }
            }
            if triplet && pair {
                sum += 1;
            }
        }
        print!(" {sum} ");
        Ok(())
    }
}

pub struct Day6 {
    pub content: String,
}
#[derive(Debug)]
enum Action {
    On,
    Off,
    Toggle,
}

impl From<&str> for Action {
    fn from(a: &str) -> Self {
        match a {
            "turn on" => Action::On,
            "turn off" => Action::Off,
            "toggle" => Action::Toggle,
            _ => panic!("Unknown acton {a}"),
        }
    }
}
#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}
#[derive(Debug)]
struct Inst {
    action: Action,
    top: Point,
    bottom: Point,
}

impl Inst {
    pub fn from(line: &str, re: &Regex) -> Self {
        let caps = re.captures(line).unwrap();
        Inst {
            action: Action::from(&caps[1]),
            top: Point {
                x: caps[2].parse::<usize>().unwrap(),
                y: caps[3].parse::<usize>().unwrap(),
            },
            bottom: Point {
                x: caps[4].parse::<usize>().unwrap(),
                y: caps[5].parse::<usize>().unwrap(),
            },
        }
    }

    pub fn act(&self, curr: &usize) -> usize {
        match self.action {
            Action::On => 1,
            Action::Off => 0,
            Action::Toggle => {
                if *curr > 0 {
                    0
                } else {
                    1
                }
            }
        }
    }
    pub fn act2(&self, curr: &usize) -> usize {
        match self.action {
            Action::On => curr + 1,
            Action::Off => {
                if *curr > 0 {
                    *curr - 1
                } else {
                    *curr
                }
            }
            Action::Toggle => curr + 2,
        }
    }
}
impl DayParts for Day6 {
    #[allow(clippy::needless_range_loop)]
    fn part1(&mut self) -> Result<(), Report> {
        let re =
            Regex::new(r"^(turn on|toggle|turn off)\s(\d+),(\d+)\sthrough\s(\d+),(\d+)").unwrap();
        let mut grid: [[usize; 1000]; 1000] = [[0; 1000]; 1000];
        for l in self.content.lines() {
            let inst = Inst::from(l, &re);
            for i in &mut grid[inst.top.x..inst.bottom.x + 1] {
                for j in &mut i[inst.top.y..inst.bottom.y + 1] {
                    *j = inst.act(j)
                }
            }
        }
        println!(
            "{}",
            grid.iter().map(|a| a.iter().sum::<usize>()).sum::<usize>()
        );
        Ok(())
    }

    fn part2(&mut self) -> Result<(), Report> {
        let re =
            Regex::new(r"^(turn on|toggle|turn off)\s(\d+),(\d+)\sthrough\s(\d+),(\d+)").unwrap();
        let mut grid: [[usize; 1000]; 1000] = [[0; 1000]; 1000];
        for l in self.content.lines() {
            let inst = Inst::from(l, &re);
            for i in &mut grid[inst.top.x..inst.bottom.x + 1] {
                for j in &mut i[inst.top.y..inst.bottom.y + 1] {
                    *j = inst.act2(j)
                }
            }
        }
        println!(
            "{}",
            grid.iter().map(|a| a.iter().sum::<usize>()).sum::<usize>()
        );
        Ok(())
    }
}

pub struct Day7 {
    pub content: String,
}

impl DayParts for Day7 {
    fn part1(&mut self) -> Result<(), Report> {
        let mut wires: HashMap<String, u16> = HashMap::new();
        let mut circuit: HashMap<String, String> = HashMap::new();
        for l in self.content.lines() {
            let input: Vec<&str> = l.split(" -> ").collect();
            let is_digit = input[0].parse::<u16>().is_ok();
            if is_digit {
                wires.insert(String::from(input[1]), input[0].parse::<u16>().unwrap());
            } else {
                circuit.insert(String::from(input[0]), String::from(input[1]));
            }
        }
        self.process(&mut circuit, &mut wires);
        Ok(())
    }

    fn part2(&mut self) -> Result<(), Report> {
        let mut wires: HashMap<String, u16> = HashMap::new();
        let mut circuit: HashMap<String, String> = HashMap::new();
        for l in self.content.lines() {
            let input: Vec<&str> = l.split(" -> ").collect();
            let is_digit = input[0].parse::<u16>().is_ok();
            if is_digit {
                wires.insert(String::from(input[1]), input[0].parse::<u16>().unwrap());
            } else {
                circuit.insert(String::from(input[0]), String::from(input[1]));
            }
        }
        wires.insert(String::from("b"), 3176);
        self.process(&mut circuit, &mut wires);
        Ok(())
    }
}

impl Day7 {
    fn process(&self, circuit: &mut HashMap<String, String>, wires: &mut HashMap<String, u16>) {
        loop {
            let mut keys_to_remove: Vec<String> = Vec::new();
            for (k, v) in circuit.iter() {
                let input: Vec<&str> = k.split(' ').collect();
                if input.len() > 2 {
                    let first = String::from(input[0]);
                    let op = input[1];
                    match op {
                        "LSHIFT" | "RSHIFT" => {
                            if wires.contains_key(&first) {
                                let n = input[2].parse::<u16>().unwrap();
                                let val = wires.get(&first).cloned();
                                if op == "LSHIFT" {
                                    wires.insert(v.clone(), val.unwrap() << n);
                                } else {
                                    wires.insert(v.clone(), val.unwrap() >> n);
                                }
                                keys_to_remove.push(k.clone());
                            }
                        }
                        "AND" | "OR" => {
                            let is_digit = input[0].parse::<u16>().is_ok();
                            if wires.contains_key(&first) || is_digit {
                                let second = String::from(input[2]);
                                let val = if is_digit {
                                    input[0].parse::<u16>().unwrap()
                                } else {
                                    wires.get(&first).cloned().unwrap()
                                };
                                if wires.contains_key(&second) {
                                    let val2 = wires.get(&second).cloned();
                                    if op == "OR" {
                                        wires.insert(v.clone(), val | val2.unwrap());
                                    } else {
                                        wires.insert(v.clone(), val & val2.unwrap());
                                    }
                                    keys_to_remove.push(k.clone());
                                }
                            }
                        }
                        _ => (),
                    }
                } else if input[0] == "NOT" {
                    let w_letter = String::from(input[1]);
                    if wires.contains_key(&w_letter) {
                        let val = wires.get(&w_letter).cloned();
                        wires.insert(v.clone(), !val.unwrap());
                        keys_to_remove.push(k.clone());
                    }
                } else {
                    let w_letter = String::from(input[0]);
                    if wires.contains_key(&w_letter) {
                        let val = wires.get(&w_letter).cloned();
                        wires.insert(v.clone(), val.unwrap());
                        keys_to_remove.push(k.clone());
                    }
                }
            }
            for k in keys_to_remove {
                circuit.remove_entry(&k);
            }
            if circuit.is_empty() {
                break;
            }
        }
        println!("Wire a {}", wires.get(&String::from("a")).unwrap());
    }
}

pub struct Day8 {
    pub content: String,
}
impl DayParts for Day8 {
    fn part1(&mut self) -> Result<(), Report> {
        let mut total = 0;
        for l in self.content.lines() {
            total += l.chars().count();
            let mut iter = l.chars();
            loop {
                match iter.next() {
                    Some('"') => (),
                    Some('\\') => match iter.next() {
                        Some('"') | Some('\\') => total -= 1,
                        Some('x') => {
                            iter.next();
                            iter.next();
                            total -= 1;
                        }
                        _ => (),
                    },
                    Some(_) => total -= 1,
                    None => break,
                }
            }
        }
        println!("{}\n", total);
        Ok(())
    }

    fn part2(&mut self) -> Result<(), Report> {
        let mut total = 0;
        let mut new_total = 0;
        for l in self.content.lines() {
            total += l.chars().count();
            let mut iter = l.chars();
            new_total += 2;
            loop {
                match iter.next() {
                    Some('"') => new_total += 2,
                    Some('\\') => match iter.next() {
                        Some('"') | Some('\\') => new_total += 4,
                        Some('x') => {
                            iter.next();
                            iter.next();
                            new_total += 5;
                        }
                        _ => (),
                    },
                    Some(_) => new_total += 1,
                    None => break,
                }
            }
        }
        println!("{}\n", new_total - total);
        Ok(())
    }
}

pub struct Day9 {
    pub content: String,
}

#[derive(Debug, Clone)]
struct Distance {
    from: String,
    to: String,
    length: usize,
}

impl Distance {
    pub fn from_line(line: &str) -> Self {
        let res: Vec<&str> = line.split(" ").collect();
        Distance {
            from: res[0].to_owned(),
            to: res[2].to_owned(),
            length: res[4].parse::<usize>().unwrap(),
        }
    }

    pub fn distance(&self, a: &str, b: &str) -> Option<usize> {
        if a == self.to && b == self.from || a == self.from && b == self.to {
            return Some(self.length);
        }
        None
    }
}

fn perm(cities: &[String]) -> Vec<Vec<String>> {
    let len = cities.len();
    let mut data = cities.to_owned();
    let mut perms: Vec<Vec<String>> = Vec::new();
    perms.push(cities.to_owned());
    let mut c = vec![0; len];
    let mut i = 0;
    loop {
        if i < len {
            if c[i] < i {
                if i % 2 == 0 {
                    data.swap(0, i);
                } else {
                    data.swap(c[i], i);
                }
                perms.push(data.to_owned());
                c[i] += 1;
                i = 0;
            } else {
                c[i] = 0;
                i += 1;
            }
        } else {
            break;
        }
    }
    perms
}
fn find_route(cities: &[String], distances: &Vec<Distance>) -> usize {
    let mut dist = 0;
    for i in 0..cities.len() - 1 {
        for d in distances {
            if let Some(l) = d.distance(&cities[i], &cities[i + 1]) {
                dist += l;
            }
        }
    }
    dist
}
impl DayParts for Day9 {
    fn part1(&mut self) -> Result<(), Report> {
        let distances: Vec<Distance> = self.content.lines().map(Distance::from_line).collect();
        let mut cities: HashSet<String> = HashSet::new();
        distances.iter().for_each(|f| {
            cities.insert(f.from.to_owned());
            cities.insert(f.to.to_owned());
        });
        let mut min: usize = usize::MAX;
        let mut arr: Vec<String> = cities.iter().map(|c| c.to_owned()).collect();
        arr.sort();
        let perm = perm(&arr);
        for p in perm {
            let r = find_route(&p, &distances);
            if min > r {
                min = r
            }
        }
        println!("{min}");
        Ok(())
    }

    fn part2(&mut self) -> Result<(), Report> {
        let distances: Vec<Distance> = self.content.lines().map(Distance::from_line).collect();
        let mut cities: HashSet<String> = HashSet::new();
        distances.iter().for_each(|f| {
            cities.insert(f.from.to_owned());
            cities.insert(f.to.to_owned());
        });
        let mut max: usize = 0;
        let mut arr: Vec<String> = cities.iter().map(|c| c.to_owned()).collect();
        arr.sort();
        let perm = perm(&arr);
        println!("{:?}", perm.len());
        for p in perm {
            let r = find_route(&p, &distances);
            if r > max {
                max = r
            }
        }
        println!("{max}");
        Ok(())
    }
}

pub struct Day10 {
    pub content: String,
}

pub fn look_and_say(input: Vec<char>) -> Vec<char> {
    let mut res: Vec<char> = Vec::new();
    if !input.is_empty() {
        let mut curr = '0';
        let mut count = 0;
        for c in input.into_iter() {
            if c == curr {
                count += 1;
            } else {
                if curr != '0' {
                    res.push(format!("{}", count).chars().next().unwrap());
                    res.push(curr);
                }
                curr = c;
                count = 1;
            }
        }
        if count > 0 {
            res.push(format!("{}", count).chars().next().unwrap());
            res.push(curr);
        }
    }
    res
}
impl DayParts for Day10 {
    fn part1(&mut self) -> Result<(), Report> {
        self.content = String::from("1113222113");
        let mut content = self.content.to_string().chars().collect();
        for _ in 0..40 {
            content = look_and_say(content);
        }
        println!("{}", content.iter().collect::<String>().len());
        Ok(())
    }

    fn part2(&mut self) -> Result<(), Report> {
        self.content = String::from("1113222113");
        let mut content = self.content.to_string().chars().collect();
        for _ in 0..50 {
            content = look_and_say(content);
        }
        println!("{}", content.iter().collect::<String>().len());
        Ok(())
    }
}

pub struct Day11 {
    pub content: String,
}
impl Day11 {
    fn inc(c: u8) -> (u8, bool) {
        let next = c + 1;
        if next > b'z' {
            return (b'a', true);
        }
        (next, false)
    }
    fn is_match(b: &[u8]) -> bool {
        let mut p1: (bool, u8, isize) = (false, 0, -1);
        let mut p2: (bool, u8, isize) = (false, 0, -1);
        let mut i = 0;
        let triple = Day11::is_triple(b);
        if !triple {
            return false;
        }
        loop {
            if i + 1 < b.len() && b[i] == b[i + 1] {
                if !p1.0 {
                    p1 = (true, b[i], i as isize);
                    i += 1;
                }
                if !p2.0 && p1.1 != b[i] {
                    p2 = (true, b[i], i as isize);
                    i += 1;
                }
            }
            i += 1;
            if i >= b.len() {
                break;
            }
        }
        triple && p1.0 && p2.0
    }
    fn is_triple(b: &[u8]) -> bool {
        let mut i = 0;
        loop {
            if b[i] == b'i' || b[i] == b'o' || b[i] == b'l' {
                return false;
            }
            if i + 2 < b.len() && (b[i] == b[i + 1] - 1 && b[i] == b[i + 2] - 2) {
                return true;
            }
            i += 1;
            if i >= b.len() {
                break;
            }
        }
        false
    }

    fn inc_string(mut b: Vec<u8>) -> Vec<u8> {
        let mut leftover = true;
        let len = b.len();
        let mut count = len - 1;

        while leftover {
            if let Some(i) = b.get_mut(count) {
                (*i, leftover) = Day11::inc(*i);
            }
            if leftover {
                if count == 0 {
                    count = len - 1;
                } else {
                    count -= 1;
                }
            } else {
                count = len - 1;
            }
        }
        b
    }
}
impl DayParts for Day11 {
    fn part1(&mut self) -> Result<(), Report> {
        let mut content = self.content.as_bytes().to_vec();
        let len = content.len();
        let mut count = len - 1;
        let mut leftover = true;
        println!("{}", String::from_utf8_lossy(&content));
        while leftover {
            if Day11::is_triple(&content) && Day11::is_match(&content) {
                break;
            }
            if let Some(i) = content.get_mut(count) {
                (*i, leftover) = Day11::inc(*i);
            }
            if leftover {
                if count == 0 {
                    count = len - 1;
                } else {
                    count -= 1;
                }
            } else {
                count = len - 1;
            }
            leftover = true;
        }
        println!("{}", String::from_utf8_lossy(&content));
        self.content = String::from_utf8_lossy(&content).to_string();
        Ok(())
    }

    fn part2(&mut self) -> Result<(), Report> {
        self.content =
            String::from_utf8_lossy(&Day11::inc_string(self.content.as_bytes().to_vec()))
                .to_string();
        self.part1()?;
        Ok(())
    }
}

pub struct Day12 {
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct Level {
    numbers: Vec<isize>,
    brace: char,
    red: bool,
}
impl Level {
    pub fn new(brace: char) -> Self {
        Level {
            brace,
            numbers: vec![],
            red: false,
        }
    }

    pub fn is_hash(&self) -> bool {
        self.brace == '}' || self.brace == '{'
    }
    pub fn add(&mut self, n: isize) {
        self.numbers.push(n);
    }
    pub fn sum(&self) -> isize {
        self.numbers.iter().sum()
    }
}
impl DayParts for Day12 {
    fn part1(&mut self) -> Result<(), Report> {
        let line = self.content.lines().next().unwrap().chars();
        let mut sum: isize = 0;
        let mut accum: Vec<char> = vec![];
        for c in line {
            if c.is_ascii_digit() || c == '-' {
                accum.push(c);
            } else if !accum.is_empty() {
                let n: isize = accum.clone().into_iter().collect::<String>().parse()?;
                sum += n;
                accum.clear();
            }
        }
        println!("{sum}");
        Ok(())
    }

    fn part2(&mut self) -> Result<(), Report> {
        let mut line = self.content.lines().next().unwrap().chars();
        let mut accum: Vec<char> = vec![];
        let mut levels: Vec<Level> = vec![];
        let mut skip_next = false;
        let mut c = None;
        let sum = loop {
            if !skip_next {
                c = line.next();
            }
            skip_next = false;
            let ch = c.unwrap();
            if ch.is_ascii_digit() || ch == '-' {
                accum.push(ch);
            } else {
                if !accum.is_empty() {
                    if let Some(a) = levels.last_mut() {
                        a.add(accum.clone().into_iter().collect::<String>().parse()?);
                    }
                    accum.clear();
                }
                if ch == '{' || ch == '[' {
                    levels.push(Level::new(ch));
                } else if ch == '}' || ch == ']' {
                    if let Some(l) = levels.pop() {
                        if levels.is_empty() {
                            break l.sum();
                        }
                        if let Some(mut l2) = levels.pop() {
                            if !l.red {
                                l2.add(l.sum());
                            }
                            levels.push(l2);
                        }
                    }
                } else if ch == '"' {
                    c = line.next();
                    skip_next = true;
                    if c.unwrap() == 'r' {
                        c = line.next();
                        if c.unwrap() == 'e' {
                            c = line.next();
                            if c.unwrap() == 'd' {
                                c = line.next();
                                if c.unwrap() == '"' {
                                    skip_next = false;
                                    if let Some(level) = levels.last_mut() {
                                        if level.is_hash() {
                                            level.red = true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        };
        println!("{sum}");
        Ok(())
    }
}

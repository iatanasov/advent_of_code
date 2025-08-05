use aoc::utils::Part;
use color_eyre::Report;
use regex::Regex;
use std::collections::HashMap;

pub fn execute(content: String, part: Part) -> Result<(), Report> {
    match part {
        Part::One => part1(content),
        Part::Two => part2(content),
    }
}

#[derive(Debug)]
pub struct CrtStack {
    stack: Vec<char>,
}

impl CrtStack {
    fn new() -> Self {
        CrtStack { stack: Vec::new() }
    }

    fn append(&mut self, crts: &mut Vec<char>) {
        self.stack.append(crts)
    }

    fn insert(&mut self, crt: char) {
        self.stack.insert(0, crt)
    }

    fn push(&mut self, crt: char) {
        self.stack.push(crt)
    }

    fn pop(&mut self) -> Option<char> {
        self.stack.pop()
    }
}

#[derive(Debug)]
pub struct CrtMove {
    move_num: usize,
    from: usize,
    to: usize,
}

impl CrtMove {
    fn new(line: &str, re: &Regex) -> Self {
        let cap = re.captures(line).unwrap();
        CrtMove {
            move_num: cap[1].parse::<usize>().unwrap(),
            from: cap[2].parse::<usize>().unwrap(),
            to: cap[3].parse::<usize>().unwrap(),
        }
    }
}

pub fn part1(content: String) -> Result<(), Report> {
    let mut manifest = true;
    let mut stack: HashMap<usize, CrtStack> = HashMap::new();
    let re = Regex::new(r"^move\s(\d+)\sfrom\s(\d+)\sto\s(\d+)").unwrap();
    for l in content.lines() {
        if l.is_empty() {
            manifest = false;
            continue;
        }
        if manifest {
            let line = l
                .replace("    ", "[-] ")
                .replace("[", " ")
                .replace("]", " ")
                .replace(" ", "");
            for (i, c) in line.chars().enumerate() {
                if c != '-' {
                    let key = i + 1;
                    match stack.get_mut(&key) {
                        Some(crt) => crt.insert(c),
                        None => {
                            let mut crt = CrtStack::new();
                            crt.insert(c);
                            stack.insert(key, crt);
                        }
                    }
                }
            }
        } else {
            let crt_move = CrtMove::new(l, &re);
            for _ in (std::ops::Range {
                start: 0,
                end: crt_move.move_num,
            }) {
                let source = stack.get_mut(&crt_move.from).unwrap().pop().unwrap();
                stack.get_mut(&crt_move.to).unwrap().push(source);
            }
        }
    }
    for i in 1..stack.len() + 1 {
        print!("{}", stack.get_mut(&i).unwrap().pop().unwrap());
    }
    Ok(())
}

pub fn part2(content: String) -> Result<(), Report> {
    let mut manifest = true;
    let mut stack: HashMap<usize, CrtStack> = HashMap::new();
    let re = Regex::new(r"^move\s(\d+)\sfrom\s(\d+)\sto\s(\d+)").unwrap();
    for l in content.lines() {
        if l.is_empty() {
            manifest = false;
            continue;
        }
        if manifest {
            let line = l
                .replace("    ", "[-] ")
                .replace("[", " ")
                .replace("]", " ")
                .replace(" ", "");
            for (i, c) in line.chars().enumerate() {
                if c != '-' {
                    let key = i + 1;
                    match stack.get_mut(&key) {
                        Some(crt) => crt.insert(c),
                        None => {
                            let mut crt = CrtStack::new();
                            crt.insert(c);
                            stack.insert(key, crt);
                        }
                    }
                }
            }
        } else {
            let crt_move = CrtMove::new(l, &re);
            let col: &mut Vec<char> = &mut Vec::new();
            for _ in (std::ops::Range {
                start: 0,
                end: crt_move.move_num,
            }) {
                let source = stack.get_mut(&crt_move.from).unwrap().pop().unwrap();
                col.insert(0, source);
            }
            stack.get_mut(&crt_move.to).unwrap().append(col);
        }
    }
    for i in 1..stack.len() + 1 {
        print!("{}", stack.get_mut(&i).unwrap().pop().unwrap());
    }
    Ok(())
}

#![allow(dead_code)]
#![allow(unused_variables)]
use aoc_2022::utils::Part;
use color_eyre::Report;
use tracing::info;

pub fn execute(content: String, part: Part) -> Result<(), Report> {
    info!("Running {:?} ", part);
    match part {
        Part::One => part1(content),
        Part::Two => part2(content),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Tile {
    y: usize,
    x: usize,
    d: char,
}

impl Tile {
    fn new(y: usize, x: usize, d: char) -> Self {
        return Tile { y, x, d };
    }
}

#[allow(unused_variables)]
pub fn part1(content: String) -> Result<(), Report> {
    let mut layout: Vec<Vec<char>> = vec![];
    for l in content.lines() {
        layout.push(l.chars().collect());
    }
    let mut start = find_s_index(&layout);
    let mut prev = start.clone();
    let mut count = 0;
    println!("{:?}", start);
    loop {
        let mut next = find_next(start, prev, &layout);
        count += 1;
        if next == None || next.unwrap().d == 'S' {
            info!("Exit because {:?}", next);
            info!("{} {}", count, count / 2 + count % 1);
            break;
        }

        println!("Next {:?}", next);
        prev = start;
        start = next.unwrap();
    }
    Ok(())
}
fn find_s_index(layout: &Vec<Vec<char>>) -> Tile {
    for (i, c) in layout.iter().enumerate() {
        for (j, d) in c.iter().enumerate() {
            if d == &'S' {
                return Tile::new(i, j, *d);
            }
        }
    }
    panic!("Fail to find S");
}

fn get_tile(y: isize, x: isize, prev: Tile, layout: &Vec<Vec<char>>) -> Option<Tile> {
    return match x >= 0
        && y >= 0
        && y < layout.len() as isize
        && x < layout[y as usize].len() as isize
    {
        true => {
            let mut t = Some(Tile::new(
                y as usize,
                x as usize,
                layout[y as usize][x as usize],
            ));
            if t.unwrap() == prev {
                t = None;
            }
            t
        }
        false => None,
    };
}
fn find_next(start: Tile, prev: Tile, layout: &Vec<Vec<char>>) -> Option<Tile> {
    let n = get_tile(start.y as isize - 1, start.x as isize, prev, layout);
    let e = get_tile(start.y as isize, start.x as isize + 1, prev, layout);
    let w = get_tile(start.y as isize, start.x as isize - 1, prev, layout);
    let s = get_tile((start.y + 1) as isize, start.x as isize, prev, layout);
    info!(
        "Checking {:?} agains N {:?} E {:?} S {:?} W {:?}",
        start, n, e, s, w
    );
    let res = match start.d {
        'S' => {
            if n != None && ['F', '7', '|'].contains(&n.unwrap().d) {
                n
            } else if e != None
                && (e.unwrap().d == 'J' || e.unwrap().d == '7' || e.unwrap().d == '-')
            {
                e
            } else if s != None
                && (s.unwrap().d == 'L' || s.unwrap().d == 'J' || s.unwrap().d == '|')
            {
                s
            } else if w != None
                && (w.unwrap().d == 'F' || w.unwrap().d == 'L' || w.unwrap().d == '-')
            {
                w
            } else {
                None
            }
        }
        '|' => {
            if n != None && ['F', '7', '|', 'S'].contains(&n.unwrap().d) {
                n
            } else if s != None && ['L', 'J', '|', 'S'].contains(&s.unwrap().d) {
                s
            } else {
                None
            }
        }
        '-' => {
            if e != None && ['J', '7', '-', 'S'].contains(&e.unwrap().d) {
                e
            } else if w != None && ['L', 'F', '-', 'S'].contains(&w.unwrap().d) {
                w
            } else {
                None
            }
        }
        'L' => {
            if n != None && n.unwrap().d != '.' && n.unwrap().d != 'L' && n.unwrap().d != 'J' {
                n
            } else if e != None && e.unwrap().d != '.' && e.unwrap().d != 'L' && e.unwrap().d != 'F'
            {
                e
            } else {
                None
            }
        }
        'J' => {
            if n != None && n.unwrap().d != '.' && n.unwrap().d != 'L' && n.unwrap().d != 'J' {
                n
            } else if w != None && w.unwrap().d != '.' && w.unwrap().d != 'J' && w.unwrap().d != '7'
            {
                w
            } else {
                None
            }
        }
        '7' => {
            if w != None && w.unwrap().d != '.' && w.unwrap().d != '|' && w.unwrap().d != 'J' {
                w
            } else if s != None
                && s.unwrap().d != '.'
                && s.unwrap().d != 'F'
                && s.unwrap().d != '-'
                && s.unwrap().d != '7'
            {
                s
            } else {
                None
            }
        }
        'F' => {
            if s != None
                && s.unwrap().d != '.'
                && s.unwrap().d != 'F'
                && s.unwrap().d != '-'
                && s.unwrap().d != '7'
            {
                s
            } else if e != None && e.unwrap().d != '.' && e.unwrap().d != '|' && e.unwrap().d != 'L'
            {
                e
            } else {
                None
            }
        }
        _ => None,
    };
    info!("Got tile {:?}", res);
    return res;
}

#[allow(unused_variables)]
pub fn part2(content: String) -> Result<(), Report> {
    Ok(())
}

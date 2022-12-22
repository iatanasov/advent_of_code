use tracing::info;
use color_eyre::Report;
use aoc_2022::utils::Part;
use std::collections::HashMap;

pub fn execute(content: String, part: Part) -> Result<(), Report> {
    match part {
        Part::One => part1(content),
        Part::Two => part2(content),
    }
}

#[derive( Debug) ]
pub struct CrtStack {
    position: usize,
    stack: Vec<char>
}

impl CrtStack {
    fn new(position: usize) -> Self {
        return CrtStack {
            position,
            stack: Vec::new() 
        }
    }
    fn insert(&mut self, crt: char)  {
        self.stack.insert(0, crt)
    }
    fn push(&mut self, crt: char) {
        self.stack.push(crt)
    }

    fn pop(&mut self) -> Option<char> {
        self.stack.pop()
    }
}

#[allow(unused_variables)]
pub fn part1(content: String) -> Result<(), Report> {
    let mut manifest = true;
    let mut stack: HashMap<usize,CrtStack> = HashMap::new();

    for l in content.lines() {
        if l.len() == 0 {
            manifest = false;
        }
        if manifest == true {
           let line = l.replace("["," ").replace("]"," ").replace("   "," ");
           for (i,c) in line.chars().enumerate() {
               if i%2 != 0 && c != ' ' {
                   let key = (i-1)/2+1;
                   if let Some(crt) =  stack.get_mut(&key) {
                       crt.insert(c)
                   } else {
                        let mut crt = CrtStack::new(key);
                        crt.insert(c);
                        stack.insert(key,crt);
                   }
                   info!("{} {} {}",key , i, c);
               }
           }
        }
    }
    info!("Stack {:?}", stack);
    Ok(())
}
#[allow(unused_variables)]
pub fn part2(content: String) -> Result<(), Report> {
    Ok(())
}

#![allow(dead_code)]
#![allow(unused_variables)]
use aoc::utils::Part;
use color_eyre::Report;
use std::cell::RefCell;
use std::rc::Rc;
use tracing::{debug, info};

use regex::Regex;

pub fn execute(content: String, part: Part) -> Result<(), Report> {
    match part {
        Part::One => part1(content),
        Part::Two => part2(content),
    }
}
#[derive(Debug, PartialEq)]
pub enum FsType {
    Dir,
    File,
}

#[derive(Debug, PartialEq)]
pub enum CmdMode {
    Ls,
    Cd,
}

#[derive(Debug)]
pub struct FsNode {
    name: String,
    size: usize,
    fs_type: FsType,
    children: Vec<Rc<RefCell<FsNode>>>,
    parent: Option<Rc<RefCell<FsNode>>>,
}

impl FsNode {
    pub fn new(
        name: String,
        size: usize,
        fs_type: FsType,
        parent: Option<Rc<RefCell<FsNode>>>,
    ) -> FsNode {
        FsNode {
            name,
            size,
            fs_type,
            parent,
            children: Vec::new(),
        }
    }

    pub fn new_file(name: String, size: usize) -> FsNode {
        FsNode {
            name,
            size,
            fs_type: FsType::File,
            parent: None,
            children: Vec::new(),
        }
    }
    pub fn add_child(&mut self, new_node: FsNode) {
        self.children.push(Rc::new(RefCell::new(new_node)));
    }

    pub fn find_child(&self, name: String) -> Option<Rc<RefCell<FsNode>>> {
        for child in &self.children {
            let c = child.borrow();
            if c.name == name {
                return Some(Rc::clone(&child));
            }
        }
        return None;
    }
    pub fn has_child(&self, name: String) -> bool {
        let mut has = false;
        for child in &self.children {
            let c = child.borrow_mut();
            if c.name == name {
                has = true;
            }
        }
        return has;
    }
    pub fn recalc_size(&mut self) -> usize {
        if self.fs_type == FsType::Dir {
            let mut size = 0;
            for child in &self.children {
                size += child.borrow_mut().recalc_size();
            }
            self.size = size;
        } else {
            debug!(
                "File {} with size {} {:?}",
                self.name, self.size, self.fs_type
            );
        }

        return self.size;
    }

    pub fn filter_by_size(&self, size: usize) -> Vec<usize> {
        let mut sizes = vec![];
        if self.size <= size {
            //info!("dir {} size {} ", self.name, self.size);
            sizes.push(self.size);
        }
        for child in &self.children {
            if self.fs_type == FsType::Dir {
                if child.borrow().fs_type == FsType::Dir {
                    sizes.append(&mut child.borrow().filter_by_size(size));
                }
            }
        }
        return sizes;
    }

    pub fn filter_by_size_ge(&self, size: usize) -> Vec<usize> {
        let mut sizes = vec![];
        if self.size >= size {
            //info!("dir {} size {} ", self.name, self.size);
            sizes.push(self.size);
        }
        for child in &self.children {
            if self.fs_type == FsType::Dir {
                if child.borrow().fs_type == FsType::Dir {
                    sizes.append(&mut child.borrow().filter_by_size_ge(size));
                }
            }
        }
        return sizes;
    }
    pub fn print_tree(&self, depth: usize) {
        //println!(" {} {} {:?} {}", "---".repeat(depth), self.name, self.fs_type, self.size);
        for child in &self.children {
            child.borrow().print_tree(depth + 1);
        }
    }
}

pub fn new_node(node: FsNode) -> Rc<RefCell<FsNode>> {
    Rc::new(RefCell::new(node))
}

pub fn build_tree(content: String) -> Rc<RefCell<FsNode>> {
    let cd_re = Regex::new(r"^\$\s+cd\s+([\w+\.]+)").unwrap();
    let ls_re = Regex::new(r"^\$\s+ls").unwrap();
    let f_re = Regex::new(r"^(dir|\d+)\s+([\w+\.]+)").unwrap();
    let mut mode = CmdMode::Cd;
    let root = new_node(FsNode::new(String::from("/"), 0, FsType::Dir, None));
    let mut cur_dir = Rc::clone(&root);
    for l in content.lines() {
        if let Some(cap) = cd_re.captures(l) {
            cur_dir = match &cap[1] {
                "/" => Rc::clone(&root),
                ".." => Rc::clone(cur_dir.borrow_mut().parent.as_ref().unwrap()),
                x => {
                    if !cur_dir.borrow_mut().has_child(String::from(x)) {
                        cur_dir.borrow_mut().add_child(FsNode::new(
                            String::from(&cap[1]),
                            0,
                            FsType::Dir,
                            Some(Rc::clone(&cur_dir)),
                        ))
                    }
                    cur_dir.borrow_mut().find_child(String::from(x)).unwrap()
                }
            };
            debug!("{:?}", &cap[1]);
            mode = CmdMode::Cd;
            continue;
        } else if ls_re.is_match(l) {
            mode = CmdMode::Ls;
        } else {
            if mode == CmdMode::Ls {
                if let Some(cap) = f_re.captures(l) {
                    if "dir" != &cap[1] {
                        cur_dir.borrow_mut().add_child(FsNode::new_file(
                            String::from(&cap[2]),
                            cap[1].parse::<usize>().unwrap(),
                        ))
                    }
                }
            }
        }
    }
    root
}

pub fn part1(content: String) -> Result<(), Report> {
    let root = build_tree(content);
    info!("Total Size: {}", root.borrow_mut().recalc_size());
    let sizes = root.borrow().filter_by_size(100000);
    root.borrow().print_tree(0);
    info!("Size : {}", sizes.iter().sum::<usize>());
    Ok(())
}

pub fn part2(content: String) -> Result<(), Report> {
    let root = build_tree(content);
    root.borrow_mut().recalc_size();
    let total = 70000000;
    let current = root.borrow().size;
    let free_space = 30000000 - (total - current);
    debug!("Space needed {}", free_space);
    let mut sizes = root.borrow().filter_by_size_ge(free_space);
    sizes.sort();
    info!("Result: {}", sizes.pop().unwrap());
    Ok(())
}

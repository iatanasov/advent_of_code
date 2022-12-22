use tracing::info;
use color_eyre::Report;
use aoc_2022::utils::Part;


pub fn execute(content: String, part: Part) -> Result<(), Report> {
    match part {
        Part::One => part1(content),
        Part::Two => part2(content),
    }
}

struct Range {
    low: usize,
    high: usize
}

impl Range {
    fn new(range: &str) -> Range {
        let pairs: Vec<&str> = range.split('-').collect();
        Range{ 
            low: pairs[0].parse::<usize>().unwrap(),
            high: pairs[1].parse::<usize>().unwrap()
        }
    }

    fn overlap(&self, other: Range) -> bool {
        if ((self.low <= other.low) && ( self.high >= other.high)) ||
            ((self.low >= other.low) && (self.high <= other.high)) {
                return true;
        }
        return false

    }
    fn no_overlap(&self, other: Range) -> bool {
        if ((self.low < other.low) && ( self.low < other.high)) &&
            ((self.high < other.low) && (self.high < other.high)) {
                return true;
        }

        if ((self.low > other.low) && ( self.low > other.high)) &&
            ((self.high > other.low) && (self.high > other.high)) {
                info!("self {} {} other {} {}", self.low, self.high,other.low, other.high); 
                return true;
        }

        return false;
    }
}

pub fn part1(content: String) -> Result<(), Report> {
    let mut count = 0;
    for l in content.lines() {
        let pairs: Vec<&str> = l.split(',').collect();
        let p1 = Range::new(pairs[0]);
        let p2 = Range::new(pairs[1]);
        if p1.overlap(p2) {
            count +=1;
        }
    }
    info!("Result {}", count);
    Ok(())
}

pub fn part2(content: String) -> Result<(), Report> {
    let mut count = 0;
    for l in content.lines() {
        let pairs: Vec<&str> = l.split(',').collect();
        let p1 = Range::new(pairs[0]);
        let p2 = Range::new(pairs[1]);
        if ! p1.no_overlap(p2) {
            count +=1;
        }
    }
    info!("Result {}", count);
    Ok(())
}

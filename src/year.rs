use clap::ValueEnum;
use color_eyre::Report;
use tracing::info;

use crate::utils::Part;

#[derive(Debug, Clone, ValueEnum)]
pub enum Year {
    Y2015,
    Y2016,
    Y2017,
    Y2018,
    Y2019,
    Y2020,
    Y2021,
    Y2022,
    Y2023,
    Y2024,
}

pub struct Day {}

pub trait DayParts {
    fn execute(&self, part: Part) -> Result<(), Report> {
        info!("Running {:?} ", part);
        match part {
            Part::One => self.part1(),
            Part::Two => self.part2(),
        }
    }
    fn part1(&self) -> Result<(), Report>;
    fn part2(&self) -> Result<(), Report>;
}

impl Year {
    pub fn to_src(&self) -> &str {
        match &self {
            Year::Y2015 => "y2015",
            Year::Y2016 => "y2016",
            Year::Y2017 => "y2017",
            Year::Y2018 => "y2018",
            Year::Y2019 => "y2019",
            Year::Y2020 => "y2020",
            Year::Y2021 => "y2021",
            Year::Y2022 => "y2022",
            Year::Y2023 => "y2023",
            Year::Y2024 => "y2024",
        }
    }
}

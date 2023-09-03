// mod day4

use std::{
    io::{BufRead, BufReader},
    ops::RangeInclusive,
    str::FromStr, fs::File,
};

struct RangePairing(RangeInclusive<i32>, RangeInclusive<i32>);

impl RangePairing {
    pub fn full_overlap(&self) -> bool {
        (self.0.start() >= self.1.start() && self.0.end() <= self.1.end())
            || (self.1.start() >= self.0.start() && self.1.end() <= self.0.end())
    }

    pub fn overlap(&self) -> bool {
        ((self.0.start() <= self.1.end()) && (self.0.start() >= self.1.start())) ||
        ((self.0.end() >= self.1.start()) && (self.0.end() <= self.1.end())) ||
        self.full_overlap()
    }
}

impl FromStr for RangePairing {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let ranges_pairs = value.split_once(',');
        let ranges_bounds = match ranges_pairs {
            Some((r1, r2)) => (r1.split_once('-'), r2.split_once('-')),
            _ => return Err("Unexpected string line data"),
        };
        let ranges_bounds_parsed = match ranges_bounds {
            (Some((r1_low, r1_upper)), Some((r2_low, r2_upper))) => (
                (i32::from_str(r1_low).ok(), i32::from_str(r1_upper).ok()),
                (i32::from_str(r2_low).ok(), i32::from_str(r2_upper).ok()),
            ),
            _ => return Err("Unexpected string line data"),
        };

        match ranges_bounds_parsed {
            ((Some(r1_low), Some(r1_upper)), (Some(r2_low), Some(r2_upper))) => {
                Ok(RangePairing(r1_low..=r1_upper, r2_low..=r2_upper))
            }
            _ => return Err("Unexpected string line data"),
        }
    }
}

fn find_fully_contained<R>(buf_reader: R) -> usize
where
    R: BufRead,
{
    buf_reader
        .lines()
        .filter_map(|line| match line.ok() {
            Some(line_str) => RangePairing::from_str(&line_str).ok(),
            _ => None,
        })
        .filter(|pair| pair.full_overlap())
        .count()
}

fn find_overlapping<R>(buf_reader: R) -> usize
where
    R: BufRead,
{
    buf_reader
        .lines()
        .filter_map(|line| match line.ok() {
            Some(line_str) => RangePairing::from_str(&line_str).ok(),
            _ => None,
        })
        .filter(|pair| pair.overlap())
        .count()
}


pub fn print_answer() {
    println!(
        "{:?}",
        find_fully_contained(BufReader::new(File::open("data/input_day4").unwrap()))
    );

    println!(
        "{:?}",
        find_overlapping(BufReader::new(File::open("data/input_day4").unwrap()))
    );
}

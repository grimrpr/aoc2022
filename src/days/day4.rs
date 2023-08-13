// mod day4

use std::{
    io::{BufRead, BufReader},
    ops::RangeInclusive,
    str::FromStr,
};

struct RangePairing(RangeInclusive<i32>, RangeInclusive<i32>);

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

fn find_fully_contained<R>(buf_reader: R)
where
    R: BufRead,
{
}

// mod day3

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::Sum;
use std::ops::Add;

pub fn print_answer() {
    println!(
        "{:?}",
        rucksack_priorities(BufReader::new(File::open("data/input_day3").unwrap()))
    );
    println!(
        "{:?}",
        rucksack_group_priorities(BufReader::new(File::open("data/input_day3").unwrap()))
    );
}

struct Rucksack {
    content: String,
}

impl Rucksack {
    pub fn contents(&self) -> &[u8] {
        self.content.trim().as_bytes()
    }

    pub fn compartments(&self) -> (&[u8], &[u8]) {
        let content_slice = self.contents();
        let compartment_size = content_slice.len();
        (
            content_slice.chunks(compartment_size / 2).next().unwrap(),
            content_slice.rchunks(compartment_size / 2).next().unwrap(),
        )
    }

    pub fn find_shared_item(&self) -> u8 {
        let (c1, c2) = self.compartments();
        *c1.iter().find(|c| c2.contains(c)).unwrap()
    }

    pub fn find_shared_by_group(&self, rucksack_1: &Rucksack, rucksack_2: &Rucksack) -> u8 {
        *self
            .contents()
            .iter()
            .find(|c| rucksack_1.contents().contains(c) && rucksack_2.contents().contains(c))
            .unwrap()
    }
}

#[derive(Copy, Clone, Debug)]
struct Priority(u32);

impl TryFrom<char> for Priority {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a'..='z' => Ok(Priority(((value as u8) - ('a' as u8) + 1) as u32)),
            'A'..='Z' => Ok(Priority(((value as u8) - ('A' as u8) + 27) as u32)),
            _ => Err("Invalid item letter for conversion"),
        }
    }
}

impl Add for Priority {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Priority(self.0 + rhs.0)
    }
}

impl Sum for Priority {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Priority(0), |acc, p| acc + p)
    }
}

fn rucksack_priorities<R>(reader: R) -> Priority
where
    R: BufRead,
{
    reader
        .lines()
        .map(|line| Rucksack {
            content: line.unwrap(),
        })
        .map(|rucksack| {
            Priority::try_from(char::try_from(rucksack.find_shared_item()).unwrap()).unwrap()
        })
        .sum()
}

fn rucksack_group_priorities<R>(reader: R) -> Priority
where
    R: BufRead,
{
    const GROUP_SIZE: usize = 3;
    let group_index = (0..GROUP_SIZE).into_iter().cycle();
    reader
        .lines()
        .map(|line| Rucksack {
            content: line.unwrap(),
        })
        .zip(group_index)
        .fold(
            (Priority(0), [None, None, None]),
            |(mut p, mut group): (Priority, [Option<Rucksack>; GROUP_SIZE]), (r, index)| {
                group[index] = Some(r);
                if index == GROUP_SIZE - 1 {
                    let shared_item = group[0].as_ref().unwrap().find_shared_by_group(
                        group[1].as_ref().unwrap(),
                        group[2].as_ref().unwrap(),
                    );
                    p = p + Priority::try_from(char::try_from(shared_item).unwrap()).unwrap();
                }
                (p, group)
            },
        )
        .0
}

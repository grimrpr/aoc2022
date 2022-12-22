// mod day2

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::Add;

pub fn print_answer() {
    println!(
        "{:?}",
        read_strategy_guide(BufReader::new(File::open("data/input_day2").unwrap()))
    );
}

#[derive(Copy, Clone)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Copy, Clone)]
enum Outcome {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

impl Shape {
    pub fn plays(&self, other: Self) -> Outcome {
        match self {
            Self::Rock => match other {
                Self::Rock => Outcome::Draw,
                Self::Scissors => Outcome::Win,
                Self::Paper => Outcome::Lose,
            },
            Self::Paper => match other {
                Self::Paper => Outcome::Draw,
                Self::Rock => Outcome::Win,
                Self::Scissors => Outcome::Lose,
            },
            Self::Scissors => match other {
                Self::Scissors => Outcome::Draw,
                Self::Paper => Outcome::Win,
                Self::Rock => Outcome::Lose,
            },
        }
    }

    pub fn self_from_outcome(result: Outcome, opponent: Self) -> Self{
        match opponent {
            Self::Rock => match result {
                Outcome::Draw => Self::Rock,
                Outcome::Win => Self::Paper,
                Outcome::Lose => Self::Scissors,
            },
            Self::Paper => match result {
                Outcome::Draw => Self::Paper,
                Outcome::Win => Self::Scissors,
                Outcome::Lose => Self::Rock,
            },
            Self::Scissors => match result {
                Outcome::Draw => Self::Scissors,
                Outcome::Win => Self::Rock,
                Outcome::Lose => Self::Paper,
            },
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Score(u32);

impl TryFrom<char> for Shape {
    type Error = &'static str;
    fn try_from(letter: char) -> Result<Self, Self::Error> {
        match letter {
            'A' | 'X' => Ok(Self::Rock),
            'B' | 'Y' => Ok(Self::Paper),
            'C' | 'Z' => Ok(Self::Scissors),
            _ => Err("Cannot decrypt into Shape from given letter"),
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = &'static str;
    fn try_from(letter: char) -> Result<Self, Self::Error> {
        match letter {
            'X' => Ok(Outcome::Lose),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err("Cannot decrypt into Outcome from given letter"),
        }
    }
}

impl From<Shape> for Score {
    fn from(value: Shape) -> Self {
        Self(value as u32)
    }
}

impl From<Outcome> for Score {
    fn from(value: Outcome) -> Self {
        Self(value as u32)
    }
}

impl Add for Score {
    type Output = Self;
    fn add(self, rhs: Score) -> Self::Output {
        Score(self.0 + rhs.0)
    }
}

fn read_strategy_guide<R>(reader: R) -> Score
where
    R: BufRead,
{
    reader
        .lines()
        .flat_map(|line| match line.unwrap().trim().as_bytes() {
            [a, _, b] => Some((char::try_from(*a).ok(), char::try_from(*b).ok())),
            _ => None,
        })
        .flat_map(|parse_tuple| match parse_tuple {
            (Some(a), Some(b)) => Some((Shape::try_from(a).unwrap(), Outcome::try_from(b).unwrap())),
            _ => None,
        })
        .map(|(opponent, result)|  (opponent, Shape::self_from_outcome(result, opponent)))
        .fold(Score(0), |score, game| {
            score + game.1.plays(game.0).into() + game.1.into()
        })
}
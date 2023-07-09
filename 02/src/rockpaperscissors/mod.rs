use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

impl RockPaperScissors {
    pub fn init(c: char) -> Self {
        match c {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!("{} has not translation to RockPaperScissors", c),
        }
    }

    pub fn score(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    pub fn play(lhs: &Self, rhs: &Self) -> (usize, usize) {
        let result = match lhs.partial_cmp(rhs).unwrap() {
            Ordering::Less => (0, 6),
            Ordering::Equal => (3, 3),
            Ordering::Greater => (6, 0),
        };
        (result.0 + lhs.score(), result.1 + rhs.score())
    }

    pub fn reparse_part2(game: &mut [(Self, Self)]) {
        for (a, b) in game {
            *b = match b {
                // X means lose
                Self::Rock => match a {
                    Self::Rock => Self::Scissors,
                    Self::Paper => Self::Rock,
                    Self::Scissors => Self::Paper,
                },
                // Y means Draw
                Self::Paper => *a,
                // Z means Win
                Self::Scissors => match a {
                    Self::Rock => Self::Paper,
                    Self::Paper => Self::Scissors,
                    Self::Scissors => Self::Rock,
                },
            }
        }
    }

    pub fn parse(filepath: &str) -> Vec<(Self, Self)> {
        // Open a file and read from it
        let file = File::open(filepath)
            .expect(&format!("Error while opening file {}", filepath));
        let reader = BufReader::new(file);

        reader
            .lines()
            .map(|l| {
                let l = l.unwrap();
                let play = l.chars().collect_vec();
                debug_assert!(play.len() == 3 && play[1] == ' ');

                (
                    RockPaperScissors::init(play[0]),
                    RockPaperScissors::init(play[2]),
                )
            })
            .collect()
    }
}

impl PartialOrd for RockPaperScissors {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self {
            Self::Rock => match other {
                Self::Rock => Ordering::Equal,
                Self::Paper => Ordering::Less,
                Self::Scissors => Ordering::Greater,
            },
            Self::Paper => match other {
                Self::Rock => Ordering::Greater,
                Self::Paper => Ordering::Equal,
                Self::Scissors => Ordering::Less,
            },
            Self::Scissors => match other {
                Self::Rock => Ordering::Less,
                Self::Paper => Ordering::Greater,
                Self::Scissors => Ordering::Equal,
            },
        })
    }
}

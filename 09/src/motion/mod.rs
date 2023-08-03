use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy)]
pub enum Motion {
    Left(u64),
    Right(u64),
    Up(u64),
    Down(u64),
}

impl Motion {
    pub fn parse(filepath: &str) -> Vec<Self> {
        // Open a file and read from it
        let file = File::open(filepath)
            .expect(&format!("Error while opening file {}", filepath));
        let reader = BufReader::new(file);

        reader
            .lines()
            .map(|l| {
                let l = l.unwrap();
                let mut atoms = l.split(" ");
                let direction = atoms.next().unwrap();
                let value = atoms.next().unwrap().parse().unwrap();

                match direction {
                    "L" => Self::Left(value),
                    "R" => Self::Right(value),
                    "U" => Self::Up(value),
                    "D" => Self::Down(value),
                    _ => panic!("Unreconizable bridge movement ()"),
                }
            })
            .collect()
    }
}

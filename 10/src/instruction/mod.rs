use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
pub enum Instruction {
    Noop,
    Add(usize, i64),
}

impl Instruction {
    pub fn parse(filepath: &str) -> Vec<Self> {
        let file = File::open(filepath)
            .expect(&format!("Error while opening file {}", filepath));
        let reader = BufReader::new(file);

        let mut result = reader
            .lines()
            .map(|l| {
                let l = l.unwrap();
                if l == "noop" {
                    Self::Noop
                } else {
                    let mut l = l.split(" ");
                    assert_eq!(l.next().unwrap(), "addx");
                    Self::Add(2, l.next().unwrap().parse().unwrap())
                }
            })
            .collect::<Vec<_>>();

        result.reverse();
        result
    }
}

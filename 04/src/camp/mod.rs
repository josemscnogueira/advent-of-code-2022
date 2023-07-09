use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct CleaningJob {
    sections: [(i32, i32); 2],
}

impl CleaningJob {
    pub fn intersection(&self) -> Option<(i32, i32)> {
        let (l, r) = (
            self.sections[0].0.max(self.sections[1].0),
            self.sections[0].1.min(self.sections[1].1),
        );

        if l <= r {
            Some((l, r))
        } else {
            None
        }
    }

    pub fn contained(&self) -> Option<usize> {
        if Self::contains(self.sections[0], self.sections[1]) {
            Some(1)
        } else if Self::contains(self.sections[1], self.sections[0]) {
            Some(0)
        } else {
            None
        }
    }

    pub fn contains(lhs: (i32, i32), rhs: (i32, i32)) -> bool {
        (lhs.0 <= rhs.0) && (lhs.1 >= rhs.1)
    }

    pub fn parse(filepath: &str) -> Vec<Self> {
        // Open a file and read from it
        let file = File::open(filepath)
            .expect(&format!("Error while opening file {}", filepath));
        let reader = BufReader::new(file);

        reader
            .lines()
            .map(|l| {
                let l = l.unwrap();
                let (left, right) = l.split_once(",").unwrap();
                let left = left.split_once("-").unwrap();
                let right = right.split_once("-").unwrap();

                Self {
                    sections: [
                        (
                            left.0.parse::<i32>().unwrap(),
                            left.1.parse::<i32>().unwrap(),
                        ),
                        (
                            right.0.parse::<i32>().unwrap(),
                            right.1.parse::<i32>().unwrap(),
                        ),
                    ],
                }
            })
            .collect()
    }
}

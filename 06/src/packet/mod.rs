use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Packet = Vec<char>;

pub fn parse(filepath: &str) -> Packet {
    // Open a file and read from it
    let file = File::open(filepath)
        .expect(&format!("Error while opening file {}", filepath));
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .next()
        .unwrap()
}

pub fn find_start(data: &Packet, length: usize) -> Option<usize> {
    let mut i = length;
    while i <= data.len() {
        let count = length - data[(i - length)..i].iter().unique().count();
        if count == 0 {
            return Some(i);
        } else {
            i += count
        }
    }
    None
}

use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse(filepath: &str) -> Vec<i32> {
    // Open a file and read from it
    let file = File::open(filepath).expect("Error while opening cave file");
    let reader = BufReader::new(file);
    let mut result = vec![0];

    reader.lines().map(|l| l.unwrap()).for_each(|l| {
        if l.is_empty() {
            result.push(0);
        } else {
            *unsafe { result.last_mut().unwrap_unchecked() } +=
                l.parse::<i32>().unwrap();
        }
    });
    if *unsafe { result.last().unwrap_unchecked() } == 0 {
        result.pop();
    }

    result.into_iter().sorted().rev().collect()
}

fn main() {
    // Parse map filepath from first argument
    let filepath = std::env::args()
        .nth(1)
        .expect("Filepath for bingo not provided");

    // Parse file
    let elfs = parse(&filepath);
    println!("1: {}", elfs.first().unwrap());
    println!("2: {}", elfs[0..3].iter().sum::<i32>());
}

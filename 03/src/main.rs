use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Compartment = Vec<char>;

#[derive(Debug)]
struct Rucksack(Compartment, Compartment);

impl Rucksack {
    fn parse(filepath: &str) -> Vec<Rucksack> {
        // Open a file and read from it
        let file = File::open(filepath)
            .expect(&format!("Error while opening file {}", filepath));
        let reader = BufReader::new(file);

        reader
            .lines()
            .map(|l| {
                let l = l.unwrap();
                let items = l.chars().collect_vec();
                let mid = items.len() / 2;
                debug_assert!(items.len() % 2 == 0);

                Self(items[..mid].to_vec(), items[mid..].to_vec())
            })
            .collect()
    }

    fn common(&self) -> Option<char> {
        for c in self.0.iter() {
            if self.1.contains(c) {
                return Some(*c);
            }
        }
        None
    }

    fn elements(&self) -> impl Iterator<Item = &char> {
        self.0.iter().chain(self.1.iter())
    }

    fn score(value: Option<char>) -> usize {
        if let Some(value) = value {
            let baseline = if value.is_lowercase() {
                ('a' as usize, 1)
            } else {
                ('A' as usize, 27)
            };
            baseline.1 + (value as usize - baseline.0)
        } else {
            0
        }
    }

    fn badges(bags: &[Self]) -> Vec<Option<char>> {
        debug_assert!(bags.len() % 3 == 0);

        let mut result = Vec::new();
        for gid in (0..bags.len()).step_by(3) {
            let mut common = None;

            let a = bags[gid + 1].elements().collect_vec();
            let b = bags[gid + 2].elements().collect_vec();
            for c in bags[gid].elements() {
                if a.contains(&c) && b.contains(&c) {
                    common = Some(*c);
                    break;
                }
            }
            result.push(common);
        }

        debug_assert!(result.len() == (bags.len() / 3));
        result
    }
}

fn main() {
    // Parse map filepath from first argument
    let filepath = std::env::args()
        .nth(1)
        .expect("Filepath for bingo not provided");

    let ruckstacks = Rucksack::parse(&filepath);
    println!(
        "1: {}",
        ruckstacks
            .iter()
            .map(|r| Rucksack::score(r.common()))
            .sum::<usize>()
    );

    println!(
        "2: {}",
        Rucksack::badges(&ruckstacks)
            .into_iter()
            .map(|r| Rucksack::score(r))
            .sum::<usize>()
    );
}

use itertools::Itertools;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy)]
pub enum CraneType {
    C9000,
    C9001,
}

#[derive(Debug)]
struct Move {
    src: usize,
    dst: usize,
    num: usize,
}

#[derive(Debug)]
pub struct SupplyStack {
    stacks: Vec<Vec<char>>,
    moves: Vec<Move>,
}

impl SupplyStack {
    pub fn apply_move(&mut self, crane: CraneType) -> bool {
        if let Some(m) = self.moves.pop() {
            let mut stack = Vec::new();
            for _ in 0..m.num {
                stack.push(self.stacks[m.src].pop().unwrap());
            }

            for c in match crane {
                CraneType::C9000 => stack,
                CraneType::C9001 => stack.into_iter().rev().collect(),
            } {
                self.stacks[m.dst].push(c);
            }

            true
        } else {
            false
        }
    }

    pub fn apply(&mut self, crane: CraneType) -> &mut Self {
        while self.apply_move(crane) {}
        self
    }

    pub fn max_height(&self) -> String {
        self.stacks
            .iter()
            .map(|s| *s.last().unwrap_or(&' '))
            .collect()
    }

    pub fn parse(filepath: &str) -> Self {
        // Open a file and read from it
        let file = File::open(filepath)
            .expect(&format!("Error while opening file {}", filepath));
        let reader = BufReader::new(file);
        let mut parsing_crates = true;
        let mut stacks = Vec::new();
        let mut moves = Vec::new();

        reader.lines().map(|l| l.unwrap()).for_each(|l| {
            if parsing_crates {
                if l.is_empty() {
                    parsing_crates = false;
                } else {
                    let repr = l.chars().collect_vec();
                    let crates = (0..repr.len()).step_by(4).map(|i| {
                        let c = repr[i + 1];
                        if c == ' ' || c.is_digit(10) {
                            None
                        } else {
                            Some(c)
                        }
                    });

                    for (i, c) in crates.into_iter().enumerate() {
                        while i >= stacks.len() {
                            stacks.push(Vec::new())
                        }
                        if let Some(c) = c {
                            stacks[i].push(c);
                        }
                    }
                }
            } else {
                let captures = Regex::new(r"^move (\d*) from (\d*) to (\d*)")
                    .unwrap()
                    .captures(&l)
                    .expect(
                        format!("input data does not match regex: {}", l)
                            .as_ref(),
                    );

                moves.push(Move {
                    src: captures[2].parse::<usize>().unwrap() - 1,
                    dst: captures[3].parse::<usize>().unwrap() - 1,
                    num: captures[1].parse::<usize>().unwrap(),
                });
            }
        });

        Self {
            stacks: stacks
                .into_iter()
                .map(|v| v.into_iter().rev().collect_vec())
                .collect_vec(),
            moves: moves.into_iter().rev().collect_vec(),
        }
    }
}

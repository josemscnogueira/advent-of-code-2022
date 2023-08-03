use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

use super::filesystem::Inode;

pub fn parse(filepath: &str) -> Inode {
    // Open a file and read from it
    let file = File::open(filepath)
        .expect(&format!("Error while opening file {}", filepath));
    let reader = BufReader::new(file);
    let mut cwd = Vec::<String>::new();
    let mut result = Inode::new_dir();
    let mut lines = reader.lines().map(|l| l.unwrap());
    let mut is_ls = false;

    while let Some(line) = lines.next() {
        let splits = line.split(" ").collect_vec();

        // If we have a dollar sign, we have a command
        let lead = splits[0];
        if lead == "$" {
            is_ls = false;

            // Command parser
            match splits[1] {
                "cd" => match splits[2] {
                    ".." => {
                        cwd.pop().unwrap();
                    }
                    "/" => cwd.clear(),
                    a => cwd.push(a.to_string()),
                },
                "ls" => {
                    is_ls = true;
                }
                c => panic!("Unrecognized command {}", c),
            }
        } else {
            assert!(is_ls);
            let path = cwd.iter().map(|c| c.as_str()).collect_vec();
            if lead == "dir" {
                result.add(&path, splits[1].to_string(), Inode::new_dir())
            } else {
                result.add(
                    &path,
                    splits[1].to_string(),
                    Inode::new_file(splits[0].parse().unwrap()),
                )
            }
            .unwrap();
        }
    }

    result
}

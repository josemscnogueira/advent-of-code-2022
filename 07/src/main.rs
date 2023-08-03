use itertools::Itertools;

mod commands;
mod filesystem;

fn main() {
    // Parse map filepath from first argument
    let filepath = std::env::args()
        .nth(1)
        .expect("Filepath for bingo not provided");

    let filesystem = commands::parse(&filepath);
    println!(
        "1: {}",
        filesystem
            .get_directories()
            .into_iter()
            .map(|d| d.size())
            .filter(|&n| n <= 100000)
            .sum::<usize>()
    );

    const STORAGE: usize = 70000000;
    const FREE_SPACE: usize = 30000000;
    const TARGET_SPACE: usize = STORAGE - FREE_SPACE;
    let total_size = filesystem.size();
    let target_delete = total_size - TARGET_SPACE;
    println!(
        "2: {}",
        filesystem
            .get_directories()
            .into_iter()
            .map(|d| d.size())
            .filter(|&n| n >= target_delete)
            .sorted()
            .next()
            .unwrap()
    );
}

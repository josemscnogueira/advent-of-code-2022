mod camp;

fn main() {
    // Parse map filepath from first argument
    let filepath = std::env::args()
        .nth(1)
        .expect("Filepath for bingo not provided");

    let camp = camp::CleaningJob::parse(&filepath);
    println!("1: {}", camp.iter().filter_map(|c| c.contained()).count());
    println!(
        "2: {}",
        camp.iter().filter_map(|c| c.intersection()).count()
    );
}

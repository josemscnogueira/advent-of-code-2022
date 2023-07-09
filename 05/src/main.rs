mod stack;

use stack::*;

fn main() {
    // Parse map filepath from first argument
    let filepath = std::env::args()
        .nth(1)
        .expect("Filepath for bingo not provided");

    println!(
        "1: {:?}",
        SupplyStack::parse(&filepath)
            .apply(CraneType::C9000)
            .max_height()
    );

    println!(
        "2: {:?}",
        SupplyStack::parse(&filepath)
            .apply(CraneType::C9001)
            .max_height()
    );
}

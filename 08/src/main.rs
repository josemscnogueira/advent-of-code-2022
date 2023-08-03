mod treehouse;

use treehouse::Treehouse;

fn main() {
    // Parse map filepath from first argument
    let filepath = std::env::args()
        .nth(1)
        .expect("Filepath for bingo not provided");

    let house = Treehouse::parse(&filepath);
    println!("1: {:?}", house.count_visible());
    println!("2: {:?}", house.scenic.iter().max().unwrap());
}

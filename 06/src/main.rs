mod packet;

fn main() {
    // Parse map filepath from first argument
    let filepath = std::env::args()
        .nth(1)
        .expect("Filepath for bingo not provided");

    let p = packet::parse(&filepath);
    println!("1: {}", packet::find_start(&p, 4).unwrap());
    println!("2: {}", packet::find_start(&p, 14).unwrap());
}

mod rockpaperscissors;

use rockpaperscissors::RockPaperScissors;

fn main() {
    // Parse map filepath from first argument
    let filepath = std::env::args()
        .nth(1)
        .expect("Filepath for bingo not provided");

    let mut game = RockPaperScissors::parse(&filepath);
    println!(
        "1: {:?}",
        game.iter()
            .map(|(a, b)| RockPaperScissors::play(&a, &b).1)
            .sum::<usize>()
    );

    RockPaperScissors::reparse_part2(&mut game);
    println!(
        "2: {:?}",
        game.iter()
            .map(|(a, b)| RockPaperScissors::play(&a, &b).1)
            .sum::<usize>()
    );
}

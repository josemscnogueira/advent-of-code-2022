mod bridge;
mod motion;

use itertools::Itertools;

use bridge::Bridge;
use motion::Motion;

fn main() {
    // Parse map filepath from first argument
    let filepath = std::env::args()
        .nth(1)
        .expect("Filepath for bingo not provided");

    let motion = Motion::parse(&filepath);
    println!(
        "1: {:?}",
        Bridge::<1>::apply_moveset(&motion)
            .trail
            .into_iter()
            .unique()
            .count()
    );
    println!(
        "2: {:?}",
        Bridge::<9>::apply_moveset(&motion)
            .trail
            .into_iter()
            .unique()
            .count()
    );
}

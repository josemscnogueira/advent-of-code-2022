mod clock;
mod instruction;

use clock::Clock;
use instruction::Instruction;

fn main() {
    // Parse map filepath from first argument
    let filepath = std::env::args()
        .nth(1)
        .expect("Filepath for bingo not provided");

    let mut i1 = Instruction::parse(&filepath);
    let mut i2 = i1.clone();
    let mut clock = Clock::init();
    println!(
        "1: {:?}",
        [20, 40, 40, 40, 40, 40]
            .into_iter()
            .map(|cycles| {
                clock.run(&mut i1, cycles);
                (clock.cycle, clock.register)
            })
            .map(|(a, b)| a as i64 * b)
            .sum::<i64>()
    );

    clock = Clock::init();
    for _ in 0..6 {
        println!(
            "2: {:?}",
            (0..40)
                .map(|_| {
                    clock.run(&mut i2, 1);
                    if clock.draw {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect::<String>()
        );
    }
}

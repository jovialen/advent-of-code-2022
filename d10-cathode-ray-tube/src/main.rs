mod communicator;
use communicator::{Communicator, Instruction};

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| line.split_whitespace())
        .map(|mut split| match split.next().unwrap_or("noop") {
            "addx" => Instruction::Addx(split.next().unwrap().parse().unwrap()),
            "noop" => Instruction::Noop,
            _ => unreachable!(),
        })
        .collect()
}

fn main() {
    let input = parse_input(include_str!("../input.txt"));
    let mut communicator = Communicator::new();

    communicator.run(&input);

    let sum_signal_strengths: i64 = communicator.get_signal_strengths().iter().sum();

    println!("Sum of signal strengths: {}", sum_signal_strengths);
    println!("Display:");
    communicator.print_display();
}

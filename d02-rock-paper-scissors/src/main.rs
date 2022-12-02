mod rps;
use rps::{Round, Scorable, Sign};

fn parse_input(input: &str) -> Vec<Round> {
    input
        .lines()
        .into_iter()
        .map(|line| {
            let mut iter = line.split_ascii_whitespace();

            let opponent = match iter.next().unwrap() {
                "A" => Sign::Rock,
                "B" => Sign::Paper,
                "C" => Sign::Scissors,
                _ => unreachable!(),
            };

            let you = match iter.next().unwrap() {
                "X" => Sign::Rock,
                "Y" => Sign::Paper,
                "Z" => Sign::Scissors,
                _ => unreachable!(),
            };

            Round::new(you, opponent)
        })
        .collect()
}

fn main() {
    let input = parse_input(include_str!("../input.txt"));
    let result = input.iter().fold(0, |acc, v| acc + v.points());
    println!("Total points: {}", result);
}

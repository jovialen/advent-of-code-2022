mod rps;
use rps::{Result, Round, Scorable, Sign};

fn parse_input_1(input: &str) -> Vec<Round> {
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

fn parse_input_2(input: &str) -> Vec<Round> {
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

            let result = match iter.next().unwrap() {
                "X" => Result::Lose,
                "Y" => Result::Draw,
                "Z" => Result::Win,
                _ => unreachable!(),
            };

            // Get the correct sign to get the desired result
            let you = match result {
                Result::Win => opponent.looses_against(),
                Result::Draw => opponent,
                Result::Lose => opponent.wins_against(),
            };

            Round::new(you, opponent)
        })
        .collect()
}

fn get_total_points(rounds: Vec<Round>) -> u64 {
    rounds.iter().map(|v| v.points()).sum()
}

fn main() {
    let raw = include_str!("../input.txt");

    let input = parse_input_1(raw);
    let result = get_total_points(input);
    println!("If XYZ are signs, then the total is: {}", result);

    let input = parse_input_2(raw);
    let result = get_total_points(input);
    println!("If XYZ are results, then the total is: {}", result);
}

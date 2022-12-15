#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position(i64, i64);

impl Position {
    fn distance(&self, other: Position) -> i64 {
        (other.0 - self.0).abs() + (other.1 - self.1).abs()
    }
}

#[derive(Debug)]
struct Sensor {
    position: Position,
    beacon: Position,
    distance: i64,
}

impl Sensor {
    fn new(position: Position, beacon: Position) -> Self {
        Self {
            position,
            beacon,
            distance: beacon.distance(position),
        }
    }
}

fn parse_input(input: &str) -> Vec<Sensor> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .filter(|c| c.is_whitespace() || c.is_digit(10) || *c == '-')
                .collect::<String>()
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|nums| {
            Sensor::new(
                Position(*nums.get(0).unwrap(), *nums.get(1).unwrap()),
                Position(*nums.get(2).unwrap(), *nums.get(3).unwrap()),
            )
        })
        .collect()
}

fn main() {
    let sensors = parse_input(include_str!("../input.txt"));
    let y = 2_000_000;

    let min_x = sensors.iter().map(|sensor| sensor.beacon.0).min().unwrap();
    let max_x = sensors.iter().map(|sensor| sensor.beacon.0).max().unwrap();

    let count = (min_x..=max_x)
        .map(|x| {
            let pos = Position(x, y);
            sensors.iter().any(|sensor| {
                sensor.position.distance(pos) <= sensor.distance
                    && pos != sensor.beacon
                    && pos != sensor.position
            })
        })
        .filter(|&is_blocked| is_blocked)
        .count();

    println!("Count of blocked tiles: {}", count);
}

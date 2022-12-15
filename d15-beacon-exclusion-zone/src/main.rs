use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    let size = 4_000_000;

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

    let points_of_interest = sensors
        .iter()
        .flat_map(|sensor| {
            let min_x = sensor.position.0 - sensor.distance;
            let max_x = sensor.position.0 + sensor.distance - 1;
            let min_y = sensor.position.1 - sensor.distance;
            let max_y = sensor.position.1 + sensor.distance - 1;

            (min_x..=max_x)
                .chain((min_x..max_x).rev())
                .zip(
                    (min_y..=max_y)
                        .cycle()
                        .skip_while(|&i| i != sensor.position.1 - 1),
                )
                .filter(|&(x, y)| x <= size && y <= size && x >= 0 && y >= 0)
                .map(|(x, y)| Position(x, y))
        })
        .collect::<HashSet<_>>();

    let distress_beacon_pos = points_of_interest
        .iter()
        .find(|point| {
            !sensors
                .iter()
                .any(|sensor| sensor.position.distance(**point) <= sensor.distance)
        })
        .unwrap();

    let tuning_freq = distress_beacon_pos.0 * 4000000 + distress_beacon_pos.1;

    println!("Tuning frequency of distress beacon: {}", tuning_freq);
}

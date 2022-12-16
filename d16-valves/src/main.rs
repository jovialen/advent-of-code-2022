#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: usize,
    connections: Vec<String>,
}

fn parse_input(input: &str) -> Vec<Valve> {
    input
        .lines()
        .filter_map(|line| line.split_once("; "))
        .map(|(left, right)| Valve {
            name: left.split_whitespace().skip(1).next().unwrap().to_string(),
            flow_rate: left.split_once("=").unwrap().1.parse().unwrap(),
            connections: right
                .split(',')
                .map(|s| s[s.len() - 2..].to_string())
                .collect(),
        })
        .collect()
}

fn main() {
    let network = parse_input(include_str!("../input.txt"));
}

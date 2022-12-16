use itertools::Itertools;

#[derive(Debug, Clone, Hash, PartialEq)]
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

fn measure_distances(network: &Vec<Valve>) -> Vec<Vec<usize>> {
    let mut distances = vec![vec![usize::MAX / 2; network.len()]; network.len()];

    for (i, valve) in network.iter().enumerate() {
        for (j, _) in network
            .iter()
            .enumerate()
            .filter(|(_, dest)| valve.connections.contains(&dest.name))
        {
            distances[i][j] = 1;
        }
        distances[i][i] = 0;
    }

    for k in 0..distances.len() {
        for i in 0..distances.len() {
            for j in 0..distances.len() {
                if distances[i][j] > distances[i][k] + distances[k][j] {
                    distances[i][j] = distances[i][k] + distances[k][j];
                }
            }
        }
    }

    distances
}

fn maximum_flow(
    current: &(usize, Valve),
    indexed_network: &[(usize, Valve)],
    distances: &Vec<Vec<usize>>,
    mut visited: Vec<(usize, Valve)>,
    remaining_time: usize,
) -> usize {
    visited.push(current.clone());

    current.1.flow_rate * remaining_time
        + indexed_network
            .iter()
            .filter(|node| !visited.contains(node))
            .filter_map(|node| {
                Some(maximum_flow(
                    node,
                    indexed_network,
                    distances,
                    visited.clone(),
                    remaining_time.checked_sub(distances[current.0][node.0] + 1)?,
                ))
            })
            .max()
            .unwrap_or(0)
}

fn maximum_flow_with_elephant(
    origin: &(usize, Valve),
    indexed_network: &[(usize, Valve)],
    distances: &Vec<Vec<usize>>,
    max_time: usize,
    teaching_time: usize,
) -> usize {
    let mut max = 0;
    for human in indexed_network
        .into_iter()
        .map(|node| node.clone())
        .combinations(indexed_network.len() / 2)
    {
        let elephant: Vec<_> = indexed_network
            .into_iter()
            .map(|node| node.clone())
            .filter(|node| !human.contains(node))
            .collect();

        max = max.max(
            maximum_flow(
                origin,
                &human,
                &distances,
                Vec::new(),
                max_time - teaching_time,
            ) + maximum_flow(
                origin,
                &elephant,
                &distances,
                Vec::new(),
                max_time - teaching_time,
            ),
        );
    }
    max
}

fn main() {
    let network = parse_input(include_str!("../input.txt"));
    let distances = measure_distances(&network);

    let reduced_network: Vec<_> = network
        .into_iter()
        .enumerate()
        .filter(|(_, valve)| valve.flow_rate > 0 || valve.name.as_str() == "AA")
        .collect();

    let origin = &reduced_network
        .iter()
        .find(|(_, valve)| valve.name.as_str() == "AA")
        .unwrap();

    let max = maximum_flow(origin, &reduced_network, &distances, Vec::new(), 30);
    println!("Maximum possible flow from valves: {}", max);

    let max = maximum_flow_with_elephant(origin, &reduced_network, &distances, 30, 4);
    println!("Maximum possible flow from valves with elephant: {}", max);
}

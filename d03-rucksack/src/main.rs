fn priority(c: char) -> u64 {
    (match c {
        'a'..='z' => c as u8 - 'a' as u8 + 1,
        'A'..='Z' => c as u8 - 'A' as u8 + 27,
        _ => unreachable!(),
    } as u64)
}

fn main() {
    let input = include_str!("../input.txt");

    let rucksack_priority_sum: u64 = input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(left, right)| left.chars().find(|&c| right.contains(c)))
        .filter_map(|v| v)
        .map(|c| priority(c))
        .sum();

    let groups_priority_sum: u64 = input
        .lines()
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .filter_map(|chunk| {
            chunk[0]
                .chars()
                .find(|&c| chunk[1].contains(c) && chunk[2].contains(c))
        })
        .map(|c| priority(c))
        .sum();

    println!("Sum of Rucksack priorities: {}", rucksack_priority_sum);
    println!("Sum of group priorities: {}", groups_priority_sum);
}

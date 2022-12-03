fn priority(c: char) -> u64 {
    (match c {
        'a'..='z' => c as u8 - 'a' as u8 + 1,
        'A'..='Z' => c as u8 - 'A' as u8 + 27,
        _ => unreachable!(),
    } as u64)
}

fn main() {
    let rucksacks: Vec<_> = include_str!("../input.txt").lines().collect();

    let rucksack_priority_sum = rucksacks
        .iter()
        .map(|line| line.split_at(line.len() / 2))
        .filter_map(|(left, right)| left.chars().find(|&c| right.contains(c)))
        .fold(0, |acc, c| acc + priority(c));

    let groups_priority_sum = rucksacks
        .chunks_exact(3)
        .filter_map(|chunk| {
            chunk[0]
                .chars()
                .find(|&c| chunk[1].contains(c) && chunk[2].contains(c))
        })
        .fold(0, |acc, c| acc + priority(c));

    println!("Sum of Rucksack priorities: {}", rucksack_priority_sum);
    println!("Sum of group priorities: {}", groups_priority_sum);
}

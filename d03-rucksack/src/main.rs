fn main() {
    let input = include_str!("../input.txt");

    let priority: u64 = input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(left, right)| left.chars().find(|&c| right.contains(c)))
        .filter_map(|v| v)
        .map(|c| match c {
            'a'..='z' => c as u8 - 'a' as u8 + 1,
            'A'..='Z' => c as u8 - 'A' as u8 + 27,
            _ => unreachable!(),
        } as u64)
        .sum();

    println!("Priority: {}", priority);
}

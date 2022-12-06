use std::collections::HashSet;

fn find_unique_sequence(bytes: &[u8], unique_count: usize) -> usize {
    bytes
        .windows(unique_count)
        .position(|window| {
            let mut unique = HashSet::new();
            window.iter().all(move |v| unique.insert(v))
        })
        .unwrap()
}

fn main() {
    let input = include_bytes!("../input.txt");

    let header_offset = find_unique_sequence(input, 4);
    let message_offset = find_unique_sequence(input, 14);

    println!("Start of packet header: {}", header_offset + 4);
    println!(
        "Header start: {:?}",
        input[header_offset..header_offset + 4]
            .iter()
            .map(|v| *v as char)
            .collect::<String>()
    );

    println!("Start of packet message: {}", message_offset + 14);
    println!(
        "Message start: {:?}",
        input[message_offset..message_offset + 14]
            .iter()
            .map(|v| *v as char)
            .collect::<String>()
    );
}

use std::collections::HashSet;

const PACKET_HEADER_START_LEN: usize = 4;
const PACKET_MESSAGE_START_LEN: usize = 14;

fn find_unique_sequence(bytes: &[u8], unique_count: usize) -> Option<(usize, String)> {
    let offset = bytes.windows(unique_count).position(|window| {
        let mut unique = HashSet::new();
        window.iter().all(move |v| unique.insert(v))
    })?;

    let sequence = bytes[offset..offset + unique_count]
        .iter()
        .map(|&v| v as char)
        .collect::<String>();

    Some((offset, sequence))
}

fn main() {
    let input = include_bytes!("../input.txt");

    if let Some((offset, seq)) = find_unique_sequence(input, PACKET_HEADER_START_LEN) {
        println!(
            "Start of packet header: {}",
            offset + PACKET_HEADER_START_LEN
        );
        println!("Header start: {}", seq);
    } else {
        println!("No packet header.");
    }

    if let Some((offset, seq)) = find_unique_sequence(input, PACKET_MESSAGE_START_LEN) {
        println!(
            "Start of packet message: {}",
            offset + PACKET_MESSAGE_START_LEN
        );
        println!("Message start: {}", seq);
    } else {
        println!("No packet message.");
    }
}

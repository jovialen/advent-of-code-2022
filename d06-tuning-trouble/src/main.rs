use std::collections::HashSet;

fn main() {
    let input = include_bytes!("../input.txt");
    let offset = input
        .windows(4)
        .position(|window| {
            let mut unique = HashSet::new();
            window.iter().all(move |v| unique.insert(v))
        })
        .unwrap();

    println!("Start of packet header: {}", offset);
    println!(
        "Header: {:?}",
        input[offset..offset + 4]
            .iter()
            .map(|v| *v as char)
            .collect::<String>()
    );
}

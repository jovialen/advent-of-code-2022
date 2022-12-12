mod a_star;
use a_star::{Map, Position};

fn main() {
    let map = Map::from(include_str!("../input.txt"));

    let len_of_path = map.a_star().expect("Failed to find a valid path").len() - 1;
    let len_of_hike = map
        .data
        .iter()
        .enumerate()
        .filter(|(_, &v)| v == 0)
        .map(|(i, _)| {
            let mut tmp_map = map.clone();
            tmp_map.start = Position::from_index(i, tmp_map.width);

            tmp_map
                .a_star()
                .and_then(|v| Some(v.len() - 1))
                .unwrap_or(usize::MAX)
        })
        .min()
        .expect("Failed to find any hike from lowest elevation to destination");

    println!(
        "Length of path to destination (no climbing): {}",
        len_of_path
    );
    println!(
        "Length of path to destination (with climbing): {}",
        len_of_hike
    );
}

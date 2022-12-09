mod rope;
use rope::*;

use rustc_hash::FxHashSet;

fn parse_input(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| line.split_whitespace())
        .filter_map(|mut split| Some((split.next()?, split.next()?.parse::<usize>().ok()?)))
        .flat_map(|(dir, count)| match dir {
            "U" => vec![Move::Up; count],
            "D" => vec![Move::Down; count],
            "L" => vec![Move::Left; count],
            "R" => vec![Move::Right; count],
            _ => unreachable!(),
        })
        .collect()
}

fn find_unique_position_count(moves: &Vec<Move>, rope_len: usize) -> usize {
    let mut rope = Rope::new(rope_len - 1);
    let mut unique = FxHashSet::from_iter([*rope.get_tail().unwrap()]);

    for m in moves {
        rope.do_move(*m);
        unique.insert(*rope.get_tail().unwrap());
    }
    unique.len()
}

fn main() {
    let input = parse_input(include_str!("../input.txt"));

    let unique_positions_rope = find_unique_position_count(&input, 2);
    let unique_positions_snapped_rope = find_unique_position_count(&input, 10);

    println!("Unique tail positions for rope: {}", unique_positions_rope);
    println!(
        "Unique tail positions for snapped rope: {}",
        unique_positions_snapped_rope
    );
}

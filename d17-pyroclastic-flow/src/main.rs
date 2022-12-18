use std::collections::HashMap;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn parse_input(input: &str) -> Vec<Direction> {
    input
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => unreachable!("{}", c),
        })
        .collect()
}

const TETRINOS: &[u32] = &[
    // ....
    // ....
    // ....
    // 1111
    0b0000_0000_0000_0000_0000_0000_1111_0000 >> 2,
    // ....
    // .1..
    // 111.
    // .1..
    0b0000_0000_0100_0000_1110_0000_0100_0000 >> 2,
    // ....
    // ..1.
    // ..1.
    // 111.
    0b0000_0000_0010_0000_0010_0000_1110_0000 >> 2,
    // 1...
    // 1...
    // 1...
    // 1...
    0b1000_0000_1000_0000_1000_0000_1000_0000 >> 2,
    // ....
    // ....
    // 11..
    // 11..
    0b0000_0000_0000_0000_1100_0000_1100_0000 >> 2,
];

fn shift_tetrino(tetrino: u32, dir: &Direction) -> u32 {
    let tetrino_mask = 0b1111_1110_1111_1110_1111_1110_1111_1110;
    let blocking_left_bits = 0b1000_0000_1000_0000_1000_0000_1000_0000;
    let blocking_right_bits = 0b0000_0010_0000_0010_0000_0010_0000_0010;
    match dir {
        Direction::Left if tetrino & blocking_left_bits == 0 => (tetrino << 1) & tetrino_mask,
        Direction::Right if tetrino & blocking_right_bits == 0 => (tetrino >> 1) & tetrino_mask,
        _ => tetrino & tetrino_mask,
    }
}

fn simulate(rock_count: usize, directions: &Vec<Direction>) -> usize {
    let mut cache = HashMap::new();
    let mut tower: Vec<u8> = vec![0xFF];
    let mut next_tetrino = TETRINOS.iter().enumerate().cycle();
    let mut next_stream = directions.iter().enumerate().cycle();
    let mut height = 0;
    let mut i = 0;
    while i < rock_count {
        let (tetrino_index, mut tetrino) = next_tetrino
            .next()
            .and_then(|(i, t)| Some((i, *t)))
            .unwrap();

        for y in (1..tower.len() + 4).rev() {
            let map_line_3 = *tower.get(y + 3).unwrap_or(&0);
            let map_line_2 = *tower.get(y + 2).unwrap_or(&0);
            let map_line_1 = *tower.get(y + 1).unwrap_or(&0);
            let map_line_0 = *tower.get(y + 0).unwrap_or(&0);
            let map_line_next = *tower.get(y - 1).unwrap_or(&0);

            let map_id = u32::from_be_bytes([map_line_3, map_line_2, map_line_1, map_line_0]);
            let next_id = u32::from_be_bytes([map_line_2, map_line_1, map_line_0, map_line_next]);

            let (direction_index, direction) = next_stream.next().unwrap();
            tetrino = match shift_tetrino(tetrino, direction) {
                res if res & map_id == 0 => res,
                _ => tetrino,
            };

            if next_id & tetrino != 0 {
                for (i, &res_line) in (map_id | tetrino).to_le_bytes().iter().enumerate() {
                    if res_line == 0 {
                        break;
                    }

                    if y + i >= tower.len() {
                        tower.push(res_line);
                        height += 1;
                    } else {
                        tower[y + i] = res_line;
                    }
                }

                let tower_id = tower
                    .iter()
                    .rev()
                    .enumerate()
                    .take(16)
                    .fold(0u128, |acc, (i, line)| acc | (*line as u128) << (i * 8));

                // Gave up: came from "https://github.com/AxlLind/AdventOfCode2022/blob/main/src/bin/17.rs"
                let key = (tetrino_index, direction_index, tower_id);
                if let Some((duration, pattern_height)) = cache.get(&key) {
                    let repeats = (rock_count - duration) / (i - duration) - 1;
                    i += (i - duration) * repeats;
                    height +=
                        (tower.iter().filter(|&&line| line != 0).count() - 1 - pattern_height)
                            * repeats;
                } else {
                    cache.insert(key, (i, height));
                }

                break;
            }
        }
        i += 1;
    }

    height
}

fn main() {
    let jetstreams = parse_input(include_str!("../input.txt"));

    let height = simulate(2022, &jetstreams);
    println!("Height of tower after 2022 blocks: {}", height);

    let height = simulate(1_000_000_000_000, &jetstreams);
    println!("Height of tower after 2022 blocks: {}", height);
}

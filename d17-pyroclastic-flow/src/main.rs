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
    // println!("Shifting {:?}", dir);
    match dir {
        Direction::Left if tetrino & blocking_left_bits == 0 => (tetrino << 1) & tetrino_mask,
        Direction::Right if tetrino & blocking_right_bits == 0 => (tetrino >> 1) & tetrino_mask,
        _ => tetrino & tetrino_mask,
    }
}

fn main() {
    let jetstreams = parse_input(include_str!("../input.txt"));
    // let jetstreams = parse_input(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");

    let mut tower: Vec<u8> = vec![0xFF];
    let mut next_stream = jetstreams.iter().cycle();
    let mut next_tetrino = TETRINOS.iter().cycle();
    for _ in 0..2022 {
        let mut tetrino = *next_tetrino.next().unwrap();
        for y in (1..tower.iter().filter(|&&line| line != 0).count() + 4).rev() {
            let map_line_3 = *tower.get(y + 3).unwrap_or(&0);
            let map_line_2 = *tower.get(y + 2).unwrap_or(&0);
            let map_line_1 = *tower.get(y + 1).unwrap_or(&0);
            let map_line_0 = *tower.get(y + 0).unwrap_or(&0);
            let map_line_next = *tower.get(y - 1).unwrap_or(&0);

            let map_id = u32::from_be_bytes([map_line_3, map_line_2, map_line_1, map_line_0]);
            let next_id = u32::from_be_bytes([map_line_2, map_line_1, map_line_0, map_line_next]);

            tetrino = match shift_tetrino(tetrino, next_stream.next().unwrap()) {
                res if res & map_id == 0 => res,
                _ => tetrino,
            };

            if next_id & tetrino != 0 {
                while tower.len() <= y + 3 {
                    tower.push(0);
                }

                let [res_line_3, res_line_2, res_line_1, res_line_0] =
                    (map_id | tetrino).to_be_bytes();

                *tower.get_mut(y + 3).unwrap() = res_line_3;
                *tower.get_mut(y + 2).unwrap() = res_line_2;
                *tower.get_mut(y + 1).unwrap() = res_line_1;
                *tower.get_mut(y + 0).unwrap() = res_line_0;

                break;
            }
        }

        // println!();
        // for line in tower.iter().rev() {
        //     println!("{}", format!("{:#010b}", line).replace("0", " "));
        // }
    }

    let height = tower.iter().filter(|&&line| line != 0).count() - 1;
    println!("Height of tower after 2022 blocks: {}", height);
}

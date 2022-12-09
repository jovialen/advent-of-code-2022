use std::collections::HashSet;

#[derive(Default, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Default, Clone, Copy)]
struct Rope {
    head: Position,
    tail: Position,
}

enum Move {
    Up(i64),
    Down(i64),
    Left(i64),
    Right(i64),
}

fn parse_input(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| line.split_whitespace())
        .filter_map(|mut split| Some((split.next()?, split.next()?.parse::<i64>().ok()?)))
        .map(|(dir, count)| match dir {
            "U" => Move::Up(count),
            "D" => Move::Down(count),
            "L" => Move::Left(count),
            "R" => Move::Right(count),
            _ => unreachable!(),
        })
        .collect()
}

fn main() {
    let input = parse_input(include_str!("../input.txt"));

    let mut rope = Rope::default();
    let mut positions = HashSet::from([rope.tail]);

    for m in input {
        let count = match m {
            Move::Up(count) => count,
            Move::Down(count) => count,
            Move::Left(count) => count,
            Move::Right(count) => count,
        };

        for _ in 0..count {
            let last_head = rope.head;

            match m {
                Move::Up(_) => rope.head.y += 1,
                Move::Down(_) => rope.head.y -= 1,
                Move::Left(_) => rope.head.x -= 1,
                Move::Right(_) => rope.head.x += 1,
            }

            let y_diff = (rope.head.y - rope.tail.y).abs();
            let x_diff = (rope.head.x - rope.tail.x).abs();

            if y_diff >= 2 || x_diff >= 2 {
                rope.tail = last_head;
                positions.insert(rope.tail);
            }
        }
    }

    println!("Unique tail positions: {}", positions.len());
}

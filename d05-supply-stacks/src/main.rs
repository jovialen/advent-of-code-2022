#[derive(Debug)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Instruction>) {
    let mut iter = input.split("\n\n");

    let stacks = iter
        .next()
        .unwrap()
        .lines()
        .rev()
        .skip(1)
        .map(|line| line.chars().skip(1).step_by(4).collect::<String>())
        .fold(Vec::new(), |mut acc, item| {
            if acc.is_empty() {
                for _ in 0..item.len() {
                    acc.push(Vec::new());
                }
            }

            for (i, v) in item.chars().enumerate() {
                if !v.is_whitespace() {
                    acc[i].push(v);
                }
            }
            acc
        });

    let instructions = iter
        .next()
        .unwrap()
        .lines()
        .map(|line| line.split_whitespace().skip(1).step_by(2))
        .flatten()
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(|chunk| Instruction {
            count: chunk[0].parse().unwrap(),
            from: chunk[1].parse().unwrap(),
            to: chunk[2].parse().unwrap(),
        })
        .collect();

    (stacks, instructions)
}

fn main() {
    let (mut stacks, instructions) = parse_input(include_str!("../input.txt"));

    // for instruction in instructions {
    //     for _ in 0..instruction.count {
    //         if let Some(v) = stacks[instruction.from - 1].pop() {
    //             stacks[instruction.to - 1].push(v);
    //         }
    //     }
    // }

    for instruction in instructions {
        let mut crates: Vec<_> = stacks[instruction.from - 1]
            .clone()
            .into_iter()
            .rev()
            .take(instruction.count)
            .rev()
            .collect();

        stacks[instruction.to - 1].append(&mut crates);
        let len = stacks[instruction.from - 1].len();
        stacks[instruction.from - 1].truncate(len - instruction.count);
    }

    let out = stacks
        .iter()
        .map(|v| v.last().unwrap_or(&' '))
        .collect::<String>();

    println!("Final stack: {}", out);
}

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
            from: chunk[1].parse::<usize>().unwrap() - 1,
            to: chunk[2].parse::<usize>().unwrap() - 1,
        })
        .collect();

    (stacks, instructions)
}

fn task_1(stacks: &Vec<Vec<char>>, instructions: &Vec<Instruction>) -> String {
    let mut stacks = stacks.clone();

    for instruction in instructions {
        let mut crates: Vec<_> = stacks[instruction.from]
            .clone()
            .into_iter()
            .rev()
            .take(instruction.count)
            .collect();

        stacks[instruction.to].append(&mut crates);
        let len = stacks[instruction.from].len();
        stacks[instruction.from].truncate(len - instruction.count);
    }

    stacks.iter().map(|v| v.last().unwrap_or(&' ')).collect()
}

fn task_2(stacks: &Vec<Vec<char>>, instructions: &Vec<Instruction>) -> String {
    let mut stacks = stacks.clone();

    for instruction in instructions {
        let mut crates: Vec<_> = stacks[instruction.from]
            .clone()
            .into_iter()
            .rev()
            .take(instruction.count)
            .rev()
            .collect();

        stacks[instruction.to].append(&mut crates);
        let len = stacks[instruction.from].len();
        stacks[instruction.from].truncate(len - instruction.count);
    }

    stacks.iter().map(|v| v.last().unwrap_or(&' ')).collect()
}

fn main() {
    let (stacks, instructions) = parse_input(include_str!("../input.txt"));

    let task_1 = task_1(&stacks, &instructions);
    let task_2 = task_2(&stacks, &instructions);

    println!("Final stack with CrateMover 9000: {}", task_1);
    println!("Final stack with CrateMover 9001: {}", task_2);
}

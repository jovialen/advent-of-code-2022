#[derive(Clone, Copy)]
enum Instruction {
    Noop,
    Addx(i64),
}

impl Instruction {
    fn len(self) -> usize {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }
}

fn execute(instructions: &Vec<Instruction>, breakpoints: &[usize]) -> Vec<i64> {
    let mut cycle = 0;
    let mut register_x: i64 = 1;
    let mut signal_strengths = Vec::new();

    for instruction in instructions {
        for _ in 0..instruction.len() {
            cycle += 1;

            if breakpoints.contains(&cycle) {
                signal_strengths.push(register_x * cycle as i64);
            }
        }

        match instruction {
            Instruction::Noop => (),
            Instruction::Addx(v) => register_x += v,
        }
    }

    signal_strengths
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| line.split_whitespace())
        .map(|mut split| match split.next().unwrap_or("noop") {
            "addx" => Instruction::Addx(split.next().unwrap().parse().unwrap()),
            "noop" => Instruction::Noop,
            _ => unreachable!(),
        })
        .collect()
}

fn main() {
    let input = parse_input(include_str!("../input.txt"));

    let sum_signal_strengths: i64 = execute(&input, &[20, 60, 100, 140, 180, 220]).iter().sum();
    println!("Sum of signal strengths: {}", sum_signal_strengths);
}

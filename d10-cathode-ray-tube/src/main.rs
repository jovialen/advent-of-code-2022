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

fn execute(instructions: &Vec<Instruction>, breakpoints: &[usize]) -> (Vec<i64>, [bool; 40 * 6]) {
    let mut cycle = 0;
    let mut register_x: i64 = 1;
    let mut signal_strengths = Vec::new();
    let mut display = [false; 40 * 6];

    for instruction in instructions {
        for _ in 0..instruction.len() {
            let xpos = (cycle % 40) as i64;
            if (register_x - 1..=register_x + 1).contains(&xpos) {
                display[cycle] = true;
            }

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

    (signal_strengths, display)
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

    let (signal_strengths, display) = execute(&input, &[20, 60, 100, 140, 180, 220]);
    let sum_signal_strengths: i64 = signal_strengths.iter().sum();
    println!("Sum of signal strengths: {}", sum_signal_strengths);

    for i in 0..6 {
        let row = &display[i * 40..(i + 1) * 40];

        for &pixel in row {
            if pixel {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

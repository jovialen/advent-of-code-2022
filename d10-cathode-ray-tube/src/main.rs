const DISPLAY_WIDTH: usize = 40;
const DISPLAY_HEIGHT: usize = 6;
const DISPLAY_SIZE: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT;

const PIXEL_ON: char = '#';
const PIXEL_OFF: char = ' ';

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

struct Communicator {
    cycle: usize,
    x_register: i64,
    display: [bool; DISPLAY_SIZE],
    signal_strengths: Vec<i64>,
}

impl Communicator {
    fn new() -> Self {
        Self {
            cycle: 0,
            x_register: 1,
            display: [false; DISPLAY_SIZE],
            signal_strengths: Vec::new(),
        }
    }

    fn run(&mut self, instructions: &Vec<Instruction>) {
        self.signal_strengths.clear();
        for inst in instructions {
            self.interpret(inst);
        }
    }

    fn print_display(&self) {
        for y in 0..DISPLAY_HEIGHT {
            let row = self.display[y * DISPLAY_WIDTH..(y + 1) * DISPLAY_WIDTH]
                .iter()
                .map(|&on| if on { PIXEL_ON } else { PIXEL_OFF })
                .collect::<String>();
            println!("{}", row);
        }
    }

    fn get_signal_strengths(&self) -> &Vec<i64> {
        &self.signal_strengths
    }

    fn interpret(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.len() {
            let xpos = (self.cycle % DISPLAY_WIDTH) as i64;
            if self.x_register - 1 <= xpos && self.x_register + 1 >= xpos {
                self.display[self.cycle % DISPLAY_SIZE] = true;
            }

            if (self.cycle + 20) % 40 == 0 {
                self.signal_strengths
                    .push(self.x_register * self.cycle as i64);
            }

            self.cycle += 1;
        }

        match instruction {
            Instruction::Noop => (),
            Instruction::Addx(v) => self.x_register += v,
        }
    }
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
    let mut communicator = Communicator::new();

    communicator.run(&input);

    let sum_signal_strengths: i64 = communicator.get_signal_strengths().iter().sum();

    println!("Sum of signal strengths: {}", sum_signal_strengths);
    println!("Display:");
    communicator.print_display();
}

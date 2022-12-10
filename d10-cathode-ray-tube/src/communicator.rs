const DISPLAY_WIDTH: usize = 40;
const DISPLAY_HEIGHT: usize = 6;
const DISPLAY_SIZE: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT;

const PIXEL_ON: char = '#';
const PIXEL_OFF: char = ' ';

#[derive(Clone, Copy)]
pub enum Instruction {
    Noop,
    Addx(i64),
}

impl Instruction {
    pub fn len(self) -> usize {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }
}

pub struct Communicator {
    cycle: usize,
    x_register: i64,
    display: [bool; DISPLAY_SIZE],
    signal_strengths: Vec<i64>,
}

impl Communicator {
    pub fn new() -> Self {
        Self {
            cycle: 0,
            x_register: 1,
            display: [false; DISPLAY_SIZE],
            signal_strengths: Vec::new(),
        }
    }

    pub fn run(&mut self, instructions: &Vec<Instruction>) {
        self.signal_strengths.clear();
        for inst in instructions {
            self.interpret(inst);
        }
    }

    pub fn print_display(&self) {
        for y in 0..DISPLAY_HEIGHT {
            let row = self.display[y * DISPLAY_WIDTH..(y + 1) * DISPLAY_WIDTH]
                .iter()
                .map(|&on| if on { PIXEL_ON } else { PIXEL_OFF })
                .collect::<String>();
            println!("{}", row);
        }
    }

    pub fn get_signal_strengths(&self) -> &Vec<i64> {
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

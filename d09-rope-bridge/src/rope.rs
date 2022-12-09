#[derive(Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Position {
    x: i64,
    y: i64,
}

#[derive(Clone)]
pub struct Rope {
    head: Position,
    tail: Vec<Position>,
}

impl Rope {
    pub fn new(tail_len: usize) -> Self {
        Self {
            head: Position::default(),
            tail: vec![Position::default(); tail_len],
        }
    }

    pub fn do_move(&mut self, m: Move) {
        let last_head = self.head;

        match m {
            Move::Up => self.head.y += 1,
            Move::Down => self.head.y -= 1,
            Move::Left => self.head.x -= 1,
            Move::Right => self.head.x += 1,
        }

        self.update_tail();
    }

    pub fn get_tail(&self) -> Option<&Position> {
        self.tail.last()
    }

    fn update_tail(&mut self) {
        for i in 0..self.tail.len() {
            let parent = if i > 0 {
                *self.tail.get(i - 1).unwrap()
            } else {
                self.head
            };

            let child = self.tail.get_mut(i).unwrap();

            let dx = parent.x - child.x;
            let dy = parent.y - child.y;

            if dx.abs() >= 2 || dy.abs() >= 2 {
                child.x += dx.signum();
                child.y += dy.signum();
            }
        }
    }
}

#[derive(Clone, Copy)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

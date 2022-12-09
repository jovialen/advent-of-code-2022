#[derive(Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Position {
    x: i64,
    y: i64,
}

#[derive(Clone, Copy)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
pub struct Rope {
    knots: Vec<Position>,
}

impl Rope {
    pub fn new(tail_len: usize) -> Self {
        Self {
            knots: vec![Position::default(); tail_len + 1],
        }
    }

    pub fn do_move(&mut self, m: Move) {
        let head = self.knots.first_mut().unwrap();

        match m {
            Move::Up => head.y += 1,
            Move::Down => head.y -= 1,
            Move::Left => head.x -= 1,
            Move::Right => head.x += 1,
        }

        self.update_tail();
    }

    pub fn get_tail(&self) -> Option<&Position> {
        self.knots.last()
    }

    fn update_tail(&mut self) {
        for i in 1..self.knots.len() {
            let parent = *self.knots.get(i - 1).unwrap();
            let child = self.knots.get_mut(i).unwrap();

            let dx = parent.x - child.x;
            let dy = parent.y - child.y;

            if dx.abs() >= 2 || dy.abs() >= 2 {
                child.x += dx.signum();
                child.y += dy.signum();
            }
        }
    }
}

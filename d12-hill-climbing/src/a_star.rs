use rustc_hash::FxHashMap;

#[derive(Hash, PartialEq, Eq, Debug, Default, Clone, Copy)]
pub struct Position {
    width: usize,
    x: usize,
    y: usize,
}

impl Position {
    pub fn from_index(i: usize, width: usize) -> Self {
        let x = i % width;
        let y = i / width;
        Self { width, x, y }
    }

    pub fn to_index(&self) -> usize {
        self.x + self.y * self.width
    }

    pub fn dist(&self, other: Position) -> f64 {
        let a = (self.x as f64 - other.x as f64).powf(2.0);
        let b = (self.y as f64 - other.y as f64).powf(2.0);
        (a + b).sqrt()
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    pub data: Vec<u8>,
    pub width: usize,
    pub start: Position,
    pub destination: Position,
}

impl From<&str> for Map {
    fn from(src: &str) -> Self {
        let width = src
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.len())
            .min()
            .unwrap_or(0);
        let mut start = Position::default();
        let mut best = Position::default();
        let data = src
            .lines()
            .flat_map(|line| line.chars().take(width))
            .enumerate()
            .map(|(i, c)| match c {
                'a'..='z' => c as u8,
                'S' => {
                    start = Position::from_index(i, width);
                    'a' as u8
                }
                'E' => {
                    best = Position::from_index(i, width);
                    'z' as u8
                }
                _ => unreachable!(),
            } - 'a' as u8)
            .collect();

        Self {
            data,
            width,
            start,
            destination: best,
        }
    }
}

impl Map {
    pub fn a_star(&self) -> Option<Vec<Position>> {
        let mut open = vec![self.start];
        let mut came_from = FxHashMap::default();
        let mut g_score = FxHashMap::default();
        let mut f_score = FxHashMap::default();
        g_score.insert(self.start, 0);
        f_score.insert(self.start, self.start.dist(self.destination));

        while !open.is_empty() {
            let current = *open
                .iter()
                .min_by(|x, y| {
                    f_score
                        .get(x)
                        .unwrap_or(&f64::MAX)
                        .partial_cmp(f_score.get(y).unwrap_or(&f64::MAX))
                        .unwrap()
                })
                .unwrap();
            let current_value = self.data[current.to_index()];

            if current == self.destination {
                let mut path = vec![current];
                let mut current = current;

                while came_from.contains_key(&current) {
                    current = *came_from.get(&current).unwrap();
                    path.push(current);
                }

                return Some(path);
            }

            open.remove(open.iter().position(|&x| x == current).unwrap());

            let offsets: [isize; 4] = [-1, 1, self.width as isize, -(self.width as isize)];
            for offset in offsets {
                let neighbour = current.to_index() as isize + offset;
                let neighbour_value = self.data.get(neighbour as usize);

                if neighbour_value.is_none() {
                    continue;
                }

                let neighbour = Position::from_index(neighbour as usize, self.width);

                let delta = if current_value + 1 < *neighbour_value.unwrap() {
                    // Never climb more than one at once
                    i32::MAX
                } else {
                    1
                };

                let tentative_g_score = g_score
                    .get(&current)
                    .unwrap_or(&i32::MAX)
                    .checked_add(delta)
                    .unwrap_or(i32::MAX);

                if tentative_g_score < *g_score.get(&neighbour).unwrap_or(&i32::MAX) {
                    came_from.insert(neighbour, current);
                    g_score.insert(neighbour, tentative_g_score);
                    f_score.insert(
                        neighbour,
                        tentative_g_score as f64 + neighbour.dist(self.destination),
                    );

                    if !open.contains(&neighbour) {
                        open.push(neighbour);
                    }
                }
            }
        }

        None
    }
}

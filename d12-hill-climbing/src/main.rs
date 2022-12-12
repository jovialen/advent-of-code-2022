use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Debug, Default, Clone, Copy)]
struct Position {
    width: usize,
    x: usize,
    y: usize,
}

impl Position {
    fn from_index(i: usize, width: usize) -> Self {
        let x = i % width;
        let y = i / width;
        Self { width, x, y }
    }

    fn to_index(&self) -> usize {
        self.x + self.y * self.width
    }

    fn dist(&self, other: Position) -> f64 {
        let a = (self.x as f64 - other.x as f64).powf(2.0);
        let b = (self.y as f64 - other.y as f64).powf(2.0);
        (a + b).sqrt()
    }
}

#[derive(Debug)]
struct Map {
    data: Vec<u8>,
    width: usize,
    start: Position,
    best: Position,
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
            best,
        }
    }
}

fn a_star(map: &Map) -> Option<Vec<Position>> {
    let mut open = vec![map.start];
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();
    let mut f_score = HashMap::new();
    g_score.insert(map.start, 0);
    f_score.insert(map.start, map.start.dist(map.best));

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
        let current_value = map.data[current.to_index()];

        if current == map.best {
            let mut path = vec![current];
            let mut current = current;

            while came_from.contains_key(&current) {
                current = *came_from.get(&current).unwrap();
                path.push(current);
            }

            return Some(path);
        }

        open.remove(open.iter().position(|&x| x == current).unwrap());

        let offsets: [isize; 4] = [-1, 1, map.width as isize, -(map.width as isize)];
        for offset in offsets {
            let neighbour = current.to_index() as isize + offset;
            let neighbour_value = map.data.get(neighbour as usize);

            if neighbour_value.is_none() {
                continue;
            }

            let neighbour = Position::from_index(neighbour as usize, map.width);

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
                    tentative_g_score as f64 + neighbour.dist(map.best),
                );

                if !open.contains(&neighbour) {
                    open.push(neighbour);
                }
            }
        }
    }

    None
}

fn main() {
    let map = Map::from(include_str!("../input.txt"));

    let len_of_path = a_star(&map).expect("Failed to find a valid path").len() - 1;

    println!("Length of shortest path to destination: {}", len_of_path);
}

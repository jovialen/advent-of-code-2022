#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

type Shape = Vec<Position>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellKind {
    Air,
    Sand,
    Solid,
}

const MAX_WIDTH: usize = 800;
const MAX_HEIGHT: usize = 200;

pub struct CellMap {
    pub cells: [[CellKind; MAX_HEIGHT]; MAX_WIDTH],
    pub height: usize,
}

impl From<&str> for Position {
    fn from(src: &str) -> Self {
        let (x, y) = src.split_once(",").unwrap();
        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

impl From<&str> for CellMap {
    fn from(src: &str) -> Self {
        let mut cells = [[CellKind::Air; MAX_HEIGHT]; MAX_WIDTH];
        let mut highest_point = 0;

        let shapes: Vec<_> = src
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.split(" -> ")
                    .map(|pos| Position::from(pos))
                    .collect::<Shape>()
            })
            .collect();

        for shape in shapes {
            for slice in shape.windows(2) {
                let xs = slice.iter().map(|pos| pos.x).collect::<Vec<_>>();
                let ys = slice.iter().map(|pos| pos.y).collect::<Vec<_>>();
                let (min_x, max_x) = (*xs.iter().min().unwrap(), *xs.iter().max().unwrap());
                let (min_y, max_y) = (*ys.iter().min().unwrap(), *ys.iter().max().unwrap());

                highest_point = highest_point.max(max_y);

                if min_x == max_x {
                    for y in min_y..=max_y {
                        cells[min_x][y] = CellKind::Solid;
                    }
                } else {
                    for x in min_x..=max_x {
                        cells[x][min_y] = CellKind::Solid;
                    }
                }
            }
        }

        Self {
            cells,
            height: highest_point + 2,
        }
    }
}

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

type Shape = Vec<Position>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CellKind {
    Air,
    Sand,
    Solid,
}

const WIDTH: usize = 800;
const HEIGHT: usize = 200;
type CellMap = [[CellKind; HEIGHT]; WIDTH];

impl From<&str> for Position {
    fn from(src: &str) -> Self {
        let (x, y) = src.split_once(",").unwrap();
        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

fn parse_input(input: &str) -> (CellMap, usize) {
    let mut result = [[CellKind::Air; HEIGHT]; WIDTH];
    let mut highest_point = 0;

    let shapes: Vec<_> = input
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
                    result[min_x][y] = CellKind::Solid;
                }
            } else {
                for x in min_x..=max_x {
                    result[x][min_y] = CellKind::Solid;
                }
            }
        }
    }

    (result, highest_point + 2)
}

fn main() {
    let (mut map, height) = parse_input(include_str!("../input.txt"));

    let mut resting = 0;
    'outer: loop {
        let mut x = 500;

        for y in 0..height {
            if map[x][y + 1] == CellKind::Air {
                // Do nothing, fall straight down
            } else if map[x - 1][y + 1] == CellKind::Air {
                x -= 1;
            } else if map[x + 1][y + 1] == CellKind::Air {
                x += 1;
            } else {
                resting += 1;
                map[x][y] = CellKind::Sand;
                continue 'outer;
            }
        }

        break;
    }

    println!("Amount of sand that came to rest: {}", resting);

    map.iter_mut().for_each(|column| {
        column.iter_mut().for_each(|cell| {
            *cell = match cell {
                CellKind::Sand => CellKind::Air,
                _ => *cell,
            };
        });
    });

    let mut resting = 0;
    'outer: loop {
        if map[500][0] == CellKind::Sand {
            break;
        }

        let mut x = 500;

        for y in 0..height {
            if map[x][y + 1] == CellKind::Air {
                // Do nothing, fall straight down
            } else if map[x - 1][y + 1] == CellKind::Air {
                x -= 1;
            } else if map[x + 1][y + 1] == CellKind::Air {
                x += 1;
            } else {
                resting += 1;
                map[x][y] = CellKind::Sand;
                continue 'outer;
            }
        }

        resting += 1;
        map[x][height - 1] = CellKind::Sand;
    }

    println!("Amount of sand to block source: {}", resting);
}

mod cellmap;
use cellmap::{CellKind, CellMap};

fn main() {
    let mut map = CellMap::from(include_str!("../input.txt"));

    let mut resting = 0;
    'outer: loop {
        let mut x = 500;

        for y in 0..map.height {
            if map.cells[x][y + 1] == CellKind::Air {
                // Do nothing, fall straight down
            } else if map.cells[x - 1][y + 1] == CellKind::Air {
                x -= 1;
            } else if map.cells[x + 1][y + 1] == CellKind::Air {
                x += 1;
            } else {
                resting += 1;
                map.cells[x][y] = CellKind::Sand;
                continue 'outer;
            }
        }

        break;
    }

    println!("Amount of sand that came to rest: {}", resting);

    map.cells.iter_mut().for_each(|column| {
        column.iter_mut().for_each(|cell| {
            *cell = match cell {
                CellKind::Sand => CellKind::Air,
                _ => *cell,
            };
        });
    });

    let mut resting = 0;
    'outer: loop {
        if map.cells[500][0] == CellKind::Sand {
            break;
        }

        let mut x = 500;

        for y in 0..map.height {
            if map.cells[x][y + 1] == CellKind::Air {
                // Do nothing, fall straight down
            } else if map.cells[x - 1][y + 1] == CellKind::Air {
                x -= 1;
            } else if map.cells[x + 1][y + 1] == CellKind::Air {
                x += 1;
            } else {
                resting += 1;
                map.cells[x][y] = CellKind::Sand;
                continue 'outer;
            }
        }

        resting += 1;
        map.cells[x][map.height - 1] = CellKind::Sand;
    }

    println!("Amount of sand to block source: {}", resting);
}

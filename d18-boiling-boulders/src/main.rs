use std::{collections::HashSet, ops::Add};

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
struct Point3D(i32, i32, i32);

impl Add for Point3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

fn parse_input(input: &str) -> Vec<Point3D> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split(','))
        .filter_map(|mut xyz| {
            Some(Point3D(
                xyz.next().and_then(|x| x.trim().parse().ok())?,
                xyz.next().and_then(|y| y.trim().parse().ok())?,
                xyz.next().and_then(|z| z.trim().parse().ok())?,
            ))
        })
        .collect()
}

fn surface_area(points: Vec<Point3D>) -> usize {
    let neighbours = [
        Point3D(-1, 0, 0),
        Point3D(1, 0, 0),
        Point3D(0, -1, 0),
        Point3D(0, 1, 0),
        Point3D(0, 0, -1),
        Point3D(0, 0, 1),
    ];

    let mut surface = points.len() * neighbours.len();
    let mut visited = HashSet::new();

    for point in &points {
        visited.insert(point);

        for neighbour in neighbours {
            let cell = *point + neighbour;

            if points.contains(&cell) && !visited.contains(&cell) {
                surface -= 2;
            }
        }
    }

    surface
}

fn main() {
    let points = parse_input(include_str!("../input.txt"));

    let surface = surface_area(points);
    println!("Surface area of all shapes: {}", surface);
}

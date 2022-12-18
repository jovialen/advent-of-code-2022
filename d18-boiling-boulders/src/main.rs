use pathfinding::prelude::astar;
use std::{
    collections::HashMap,
    ops::{Add, Sub},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Point3D(i32, i32, i32);

impl Add for Point3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Point3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Point3D {
    fn get_neighbours(&self) -> [Point3D; 6] {
        [
            *self + Point3D(-1, 0, 0),
            *self + Point3D(1, 0, 0),
            *self + Point3D(0, -1, 0),
            *self + Point3D(0, 1, 0),
            *self + Point3D(0, 0, -1),
            *self + Point3D(0, 0, 1),
        ]
    }

    fn distance(&self, rhs: Self) -> f64 {
        (*self - rhs).len()
    }

    fn len(&self) -> f64 {
        ((self.0.pow(2) + self.1.pow(2) + self.2.pow(2)) as f64).sqrt()
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

fn surface_area(points: &Vec<Point3D>) -> usize {
    let mut cache = HashMap::new();

    points.iter().fold(0, move |acc, point| {
        point.get_neighbours().iter().fold(acc, |acc, neighbour| {
            let is_air = !points.contains(&neighbour);

            let goal = Point3D(-1, -1, -1);
            let is_free = if let Some(free) = cache.get(neighbour) {
                *free
            } else {
                if let Some((path, _)) = astar(
                    neighbour,
                    |point| {
                        point
                            .get_neighbours()
                            .into_iter()
                            .filter(|point| !points.contains(point))
                            .map(|point| (point, 1))
                    },
                    |point| point.distance(goal) as u32,
                    |&point| point == goal,
                ) {
                    for point in path {
                        cache.insert(point, true);
                    }
                    true
                } else {
                    cache.insert(*neighbour, false);
                    false
                }
            };

            if is_air && is_free {
                acc + 1
            } else {
                acc
            }
        })
    })
}

fn main() {
    let points = parse_input(include_str!("../input.txt"));

    let surface = surface_area(&points);
    println!("Surface area of all shapes: {}", surface);
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() + 1)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[inline]
fn is_visible(row: &Vec<u32>, column: &Vec<u32>, x: usize, y: usize, tree: u32) -> bool {
    *row[..x].iter().max().unwrap_or(&0) < tree
        || *row[x + 1..].iter().max().unwrap_or(&0) < tree
        || *column[..y].iter().max().unwrap_or(&0) < tree
        || *column[y + 1..].iter().max().unwrap_or(&0) < tree
}

fn distance_from_higher_trees(
    row: &Vec<u32>,
    column: &Vec<u32>,
    x: usize,
    y: usize,
    tree: u32,
) -> (usize, usize, usize, usize) {
    let left = row[..x]
        .iter()
        .rev()
        .scan(true, |state, &other| {
            if *state {
                *state = other < tree;
                Some(())
            } else {
                None
            }
        })
        .count();
    let right = row[x..]
        .iter()
        .skip(1)
        .scan(true, |state, &other| {
            if *state {
                *state = other < tree;
                Some(())
            } else {
                None
            }
        })
        .count();
    let top = column[..y]
        .iter()
        .rev()
        .scan(true, |state, &other| {
            if *state {
                *state = other < tree;
                Some(())
            } else {
                None
            }
        })
        .count();
    let bottom = column[y..]
        .iter()
        .skip(1)
        .scan(true, |state, &other| {
            if *state {
                *state = other < tree;
                Some(())
            } else {
                None
            }
        })
        .count();
    (left, right, top, bottom)
}

fn main() {
    let input = parse_input(include_str!("../input.txt"));
    let columns = (0..input.get(0).and_then(|v| Some(v.len())).unwrap_or(0))
        .map(|x| {
            input
                .iter()
                .map(|line| *line.get(x).unwrap_or(&0))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let count: usize = input
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|&(x, tree)| is_visible(row, &columns[x], x, y, *tree))
                .count()
        })
        .sum();
    println!("Amount of visible trees: {}", count);

    let highest_visibility = input
        .iter()
        .enumerate()
        .filter_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, tree)| {
                    let (left, right, top, bottom) =
                        distance_from_higher_trees(row, &columns[x], x, y, *tree);
                    left * right * top * bottom
                })
                .max()
        })
        .max()
        .unwrap_or(0);

    println!("Best visibility: {}", highest_visibility);
}

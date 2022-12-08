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

fn main() {
    let input = parse_input(include_str!("../input.txt"));

    let mut count = 0;
    for (y, line) in input.iter().enumerate() {
        for (x, tree) in line.iter().enumerate() {
            let column = input
                .iter()
                .map(|line| *line.get(x).unwrap_or(&0))
                .collect::<Vec<_>>();

            if line[..x].iter().max().unwrap_or(&0) < tree
                || line[x + 1..].iter().max().unwrap_or(&0) < tree
                || column[..y].iter().max().unwrap_or(&0) < tree
                || column[y + 1..].iter().max().unwrap_or(&0) < tree
            {
                count += 1;
            }
        }
    }

    println!("Amount of visible trees: {}", count);
}

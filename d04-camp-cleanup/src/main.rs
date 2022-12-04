#[derive(Debug)]
struct Assignment(usize, usize);
#[derive(Debug)]
struct Pair(Assignment, Assignment);

impl Pair {
    fn overlaps(&self) -> bool {
        (self.0 .0 <= self.1 .0 && self.0 .1 >= self.1 .1)
            || (self.1 .0 <= self.0 .0 && self.1 .1 >= self.0 .1)
    }
}

fn parse_input(input: &str) -> Vec<Pair> {
    input
        .lines()
        .map(|line| line.split(&['-', ',']))
        .flatten()
        .filter_map(|x| x.parse().ok())
        .collect::<Vec<_>>()
        .chunks_exact(4)
        .map(|chunk| {
            Pair(
                Assignment(chunk[0], chunk[1]),
                Assignment(chunk[2], chunk[3]),
            )
        })
        .collect()
}

fn main() {
    let input = parse_input(include_str!("../input.txt"));

    let contained_count = input.iter().filter(|&pair| pair.overlaps()).count();

    println!("Number of overlapping assignemnts: {}", contained_count);
}

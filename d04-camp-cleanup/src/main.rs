#[derive(Debug)]
struct Assignment(usize, usize);
#[derive(Debug)]
struct Pair(Assignment, Assignment);

impl Pair {
    fn complete_overlap(&self) -> bool {
        (self.0 .0 <= self.1 .0 && self.0 .1 >= self.1 .1)
            || (self.1 .0 <= self.0 .0 && self.1 .1 >= self.0 .1)
    }

    fn any_overlap(&self) -> bool {
        self.0 .1 >= self.1 .0 && self.0 .0 <= self.1 .1
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

    let complete_overlap_count = input.iter().filter(|&pair| pair.complete_overlap()).count();
    let any_overlap_count = input.iter().filter(|&pair| pair.any_overlap()).count();

    println!(
        "Number of completly overlapping assignemnts: {}",
        complete_overlap_count
    );

    println!(
        "Number of any overlapping assignemnts: {}",
        any_overlap_count
    );
}

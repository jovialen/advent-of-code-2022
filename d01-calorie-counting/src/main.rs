use itertools::Itertools;

/// Convert the input string to a vector.
///
/// This function converts the input file to a vector of integers storing the sum of calories each
/// elf is carrying.
fn get_calories_per_elf(input: &str) -> Vec<u64> {
    input
        .split_terminator('\n')
        .map(|x| x.trim())
        .fold(Vec::new(), |mut acc, x| {
            if x.is_empty() || acc.is_empty() {
                acc.push(Vec::new());
            }
            acc.last_mut().unwrap().push(x);
            acc
        })
        .iter()
        .map(|v| {
            v.iter()
                .fold(0, |acc, x| acc + x.parse::<u64>().unwrap_or(0))
        })
        .collect()
}

fn main() {
    let input = get_calories_per_elf(include_str!("../input.txt"));

    let max = input.iter().max().unwrap_or(&0);
    let sum = input
        .iter()
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .fold(0, |acc, &x| acc + x);

    println!("The elf carrying the most calories has {} calories.", max);
    println!(
        "The sum of the calories carried by the top 3 elves is {}",
        sum
    );
}

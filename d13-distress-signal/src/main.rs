use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;

#[derive(Debug, Clone)]
enum Element {
    Number(u64),
    List(Vec<Element>),
}

#[derive(Debug)]
struct Pair(Vec<Element>, Vec<Element>);

fn parse_element(input: &str) -> Vec<Element> {
    let mut depth = 0;
    input[1..(input.len() - 1)]
        .split(move |c| match c {
            ',' => depth == 0,
            '[' => {
                depth += 1;
                false
            }
            ']' => {
                depth -= 1;
                false
            }
            _ => false,
        })
        .map(|element| element.trim())
        .filter_map(|element| match element {
            _ if element.parse::<u64>().is_ok() => Some(Element::Number(element.parse().ok()?)),
            _ if element.starts_with('[') => Some(Element::List(parse_element(element))),
            _ => None,
        })
        .collect_vec()
}

fn parse_input(input: &str) -> Vec<Pair> {
    input
        .lines()
        .group_by(|line| !line.is_empty())
        .into_iter()
        .filter(|(not_empty, _)| *not_empty)
        .map(|(_, lines)| {
            let mut iter = lines.map(|line| parse_element(line));
            Pair(iter.next().unwrap(), iter.next().unwrap())
        })
        .collect()
}

fn compare(left: &Vec<Element>, right: &Vec<Element>) -> Option<bool> {
    for pair in left.iter().zip_longest(right.iter()) {
        match pair {
            Left(_) => return Some(false),
            Right(_) => return Some(true),
            Both(x, y) => match (x, y) {
                (Element::Number(x), Element::Number(y)) if x == y => continue,
                (Element::Number(x), Element::Number(y)) if x < y => return Some(true),
                (Element::Number(x), Element::Number(y)) if x > y => return Some(false),
                (Element::List(x), Element::Number(_)) => match compare(x, &vec![y.clone()]) {
                    res @ Some(_) => return res,
                    None => continue,
                },
                (Element::Number(_), Element::List(y)) => match compare(&vec![x.clone()], y) {
                    res @ Some(_) => return res,
                    None => continue,
                },
                (Element::List(x), Element::List(y)) => match compare(x, y) {
                    res @ Some(_) => return res,
                    None => continue,
                },
                _ => unreachable!(),
            },
        }
    }
    None
}

fn is_ordered(left: &Vec<Element>, right: &Vec<Element>) -> bool {
    compare(left, right).unwrap_or(true)
}

fn main() {
    let pairs = parse_input(include_str!("../input.txt"));

    let sum_of_indices = pairs
        .iter()
        .enumerate()
        .filter(|(_, pair)| is_ordered(&pair.0, &pair.1))
        .fold(0, |acc, (i, _)| acc + i + 1);

    println!("Sum of indices of ordered elements: {}", sum_of_indices);
}

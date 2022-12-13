use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Element {
    Number(u64),
    List(Vec<Element>),
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Element::Number(x), Element::Number(y)) => x.cmp(y),
            (Element::Number(_), Element::List(_)) => Element::List(vec![self.clone()]).cmp(other),
            (Element::List(_), Element::Number(_)) => self.cmp(&Element::List(vec![other.clone()])),
            (Element::List(x_vec), Element::List(y_vec)) => {
                for zip in x_vec.iter().zip_longest(y_vec) {
                    match zip {
                        Left(_) => return Ordering::Greater,
                        Right(_) => return Ordering::Less,
                        Both(x, y) => match x.cmp(y) {
                            Ordering::Equal => continue,
                            res @ _ => return res,
                        },
                    }
                }
                Ordering::Equal
            }
        }
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

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

fn parse_input(input: &str) -> Vec<Vec<Element>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| parse_element(line))
        .collect()
}

fn main() {
    let elements = parse_input(include_str!("../input.txt"));

    let sum_of_indices = elements
        .chunks_exact(2)
        .enumerate()
        .filter(|(_, chunk)| chunk[0] < chunk[1])
        .fold(0, |acc, (i, _)| acc + i + 1);

    println!("Sum of indices of ordered elements: {}", sum_of_indices);

    let dividers = vec![
        vec![Element::List(vec![Element::Number(2)])],
        vec![Element::List(vec![Element::Number(6)])],
    ];

    let product_of_dividers = elements
        .iter()
        .chain(dividers.iter())
        .sorted()
        .enumerate()
        .filter(|(_, element)| dividers.contains(element))
        .fold(1, |acc, (i, _)| acc * (i + 1));

    println!("Decoder key: {}", product_of_dividers);
}

use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
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

fn parse_input(input: &str) -> Vec<Vec<Element>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| parse_element(line))
        .collect()
}

fn compare(left: &Vec<Element>, right: &Vec<Element>) -> Ordering {
    for pair in left.iter().zip_longest(right.iter()) {
        match pair {
            Left(_) => return Ordering::Greater,
            Right(_) => return Ordering::Less,
            Both(x, y) => match (x, y) {
                (Element::Number(x), Element::Number(y)) if x == y => continue,
                (Element::Number(x), Element::Number(y)) if x < y => return Ordering::Less,
                (Element::Number(x), Element::Number(y)) if x > y => return Ordering::Greater,
                (Element::List(x), Element::Number(_)) => match compare(x, &vec![y.clone()]) {
                    Ordering::Equal => continue,
                    res => return res,
                },
                (Element::Number(_), Element::List(y)) => match compare(&vec![x.clone()], y) {
                    Ordering::Equal => continue,
                    res => return res,
                },
                (Element::List(x), Element::List(y)) => match compare(x, y) {
                    Ordering::Equal => continue,
                    res => return res,
                },
                _ => unreachable!(),
            },
        }
    }
    Ordering::Equal
}

fn is_ordered(left: &Vec<Element>, right: &Vec<Element>) -> bool {
    compare(left, right) != Ordering::Greater
}

fn main() {
    let elements = parse_input(include_str!("../input.txt"));

    let sum_of_indices = elements
        .chunks_exact(2)
        .enumerate()
        .filter(|(_, chunk)| is_ordered(&chunk[0], &chunk[1]))
        .fold(0, |acc, (i, _)| acc + i + 1);

    println!("Sum of indices of ordered elements: {}", sum_of_indices);

    let dividers = vec![
        vec![Element::List(vec![Element::Number(2)])],
        vec![Element::List(vec![Element::Number(6)])],
    ];

    let product_of_dividers = elements
        .iter()
        .chain(dividers.iter())
        .sorted_by(|a, b| compare(a, b))
        .enumerate()
        .filter(|(_, element)| dividers.contains(element))
        .fold(1, |acc, (i, _)| acc * (i + 1));

    println!("Decoder key: {}", product_of_dividers);
}

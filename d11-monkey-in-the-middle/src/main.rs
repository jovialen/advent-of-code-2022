use itertools::Itertools;
use std::collections::vec_deque::VecDeque;

#[derive(Debug)]
enum Operation {
    Add(u64),
    Multiply(u64),
    RaiseToPower(u32),
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test_divisible: u64,
    if_true: usize,
    if_false: usize,
    inspections: usize,
}

impl From<String> for Monkey {
    fn from(input: String) -> Self {
        let mut res = Self {
            items: VecDeque::new(),
            operation: Operation::Add(0),
            test_divisible: 1,
            if_true: 0,
            if_false: 0,
            inspections: 0,
        };

        for line in input.lines().map(|line| line.trim().split_once(": ")) {
            match line {
                Some(("Starting items", items)) => {
                    res.items = items
                        .split(", ")
                        .filter_map(|item| item.parse().ok())
                        .collect()
                }
                Some(("Operation", op)) => {
                    let split = op.split_whitespace().collect_vec();
                    res.operation = if op == "new = old * old" {
                        Operation::RaiseToPower(2)
                    } else {
                        match split.get(3) {
                            Some(&"+") => Operation::Add(split.last().unwrap().parse().unwrap()),
                            Some(&"*") => {
                                Operation::Multiply(split.last().unwrap().parse().unwrap())
                            }
                            _ => unreachable!(),
                        }
                    };
                }
                Some(("Test", test)) => {
                    res.test_divisible = test.split_whitespace().last().unwrap().parse().unwrap()
                }
                Some(("If true", to)) => {
                    res.if_true = to.split_whitespace().last().unwrap().parse().unwrap()
                }
                Some(("If false", to)) => {
                    res.if_false = to.split_whitespace().last().unwrap().parse().unwrap()
                }
                line => unreachable!("{:?}", line),
            }
        }

        res
    }
}

fn parse_input(input: &str) -> Vec<Monkey> {
    input
        .lines()
        .group_by(|line| !line.is_empty())
        .into_iter()
        .filter(|&(not_empty, _)| not_empty)
        .map(|(_, lines)| lines.into_iter().collect_vec())
        .map(|lines| Monkey::from(lines.into_iter().skip(1).join("\n")))
        .collect()
}

fn main() {
    let mut monkeys = parse_input(include_str!("../input.txt"));

    for monkey in &monkeys {
        println!("{:?}", monkey);
    }

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys.get_mut(i).unwrap().items.pop_front() {
                let worry = (match monkeys.get(i).unwrap().operation {
                    Operation::Add(x) => item + x,
                    Operation::Multiply(x) => item * x,
                    Operation::RaiseToPower(x) => item.pow(x),
                } as f64
                    / 3.0)
                    .floor() as u64;

                monkeys.get_mut(i).unwrap().inspections += 1;

                if worry % monkeys.get(i).unwrap().test_divisible == 0 {
                    let to = monkeys.get(i).unwrap().if_true;
                    monkeys.get_mut(to).unwrap().items.push_back(worry);
                } else {
                    let to = monkeys.get(i).unwrap().if_false;
                    monkeys.get_mut(to).unwrap().items.push_back(worry);
                }
            }
        }
    }

    let product: usize = monkeys
        .iter()
        .map(|monkey| monkey.inspections)
        .sorted()
        .rev()
        .take(2)
        .product();

    println!("{}", product);
}

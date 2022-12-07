use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
enum Entry {
    File(usize),
    Directory(String),
}

fn parse_input(input: &str) -> HashMap<PathBuf, Vec<Entry>> {
    let mut path = PathBuf::new();
    let mut filesystem = HashMap::new();

    for line in input.lines().filter(|line| !line.is_empty()) {
        if line.starts_with("$") {
            match line[2..].split_at(2) {
                ("cd", " ..") => {
                    path.pop();
                }
                ("cd", dirname) => {
                    path.push(dirname.trim());
                    if !filesystem.contains_key(&path) {
                        filesystem.insert(path.clone(), Vec::new());
                    }
                }
                ("ls", _) => (),
                (command, _) => unreachable!("Unknown command \"{}\"", command),
            }
        } else {
            match line.split_once(char::is_whitespace) {
                Some(("dir", dirname)) => filesystem
                    .get_mut(&path)
                    .unwrap()
                    .push(Entry::Directory(dirname.to_string())),
                Some((size, _filename)) => filesystem
                    .get_mut(&path)
                    .unwrap()
                    .push(Entry::File(size.parse().unwrap_or(0))),
                None => unreachable!(),
            }
        }
    }

    filesystem
}

fn size_of_dir(filesystem: &HashMap<PathBuf, Vec<Entry>>, dir: &PathBuf) -> usize {
    filesystem
        .get(dir)
        .unwrap()
        .iter()
        .map(|entry| match entry {
            Entry::File(size) => *size,
            Entry::Directory(name) => {
                let mut sub_dir = dir.clone();
                sub_dir.push(name);
                size_of_dir(filesystem, &sub_dir)
            }
        })
        .sum()
}

fn main() {
    let input = parse_input(include_str!("../input.txt"));

    let entry_sizes = input
        .keys()
        .map(|key| size_of_dir(&input, &key))
        .collect::<Vec<_>>();

    let sum: usize = entry_sizes.iter().filter(|&&size| size <= 100000).sum();
    println!("Total size of all dirs under size of 100000: {}", sum);

    let smallest = entry_sizes
        .iter()
        .filter(|&&size| size >= 8381165)
        .min()
        .unwrap_or(&0);
    println!("Size of the smallest directory to be deleted: {}", smallest);
}

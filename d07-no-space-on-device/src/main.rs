use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
enum Entry {
    File(usize),
    Directory(String),
}

fn parse_input(input: &str) -> HashMap<PathBuf, Vec<Entry>> {
    let mut path = PathBuf::from("/");
    let mut filesystem = HashMap::new();

    for mut line in input.lines().map(|line| line.split_ascii_whitespace()) {
        match line.next() {
            Some("$") => match line.next() {
                Some("cd") => match line.next() {
                    Some("..") => {
                        path.pop();
                    }
                    Some(dir) => {
                        path.push(dir);
                        if !filesystem.contains_key(&path) {
                            filesystem.insert(path.clone(), Vec::new());
                        }
                    }
                    None => unreachable!(),
                },
                _ => (),
            },
            Some("dir") => filesystem
                .get_mut(&path)
                .unwrap()
                .push(Entry::Directory(line.next().unwrap().to_string())),
            Some(file_size) => filesystem
                .get_mut(&path)
                .unwrap()
                .push(Entry::File(file_size.parse().unwrap())),
            None => (),
        }
    }

    filesystem
}

fn size_of_dir(filesystem: &HashMap<PathBuf, Vec<Entry>>, dir: &PathBuf) -> usize {
    let mut total_size = 0;
    for entry in filesystem.get(dir).unwrap() {
        match entry {
            Entry::File(size) => total_size += size,
            Entry::Directory(name) => {
                let mut sub_dir = dir.clone();
                sub_dir.push(name);
                total_size += size_of_dir(filesystem, &sub_dir);
            }
        }
    }
    total_size
}

fn main() {
    let input = parse_input(include_str!("../input.txt"));

    let mut sum = 0;
    for (k, _) in &input {
        let size = size_of_dir(&input, k);
        if size <= 100000 {
            sum += size;
        }
    }

    println!("Total size of all dirs under size of 100000: {}", sum);
}

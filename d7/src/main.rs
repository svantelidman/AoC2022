use std::iter::Peekable;
use std::str::Lines;

fn main() {
    let root = parse_file_system(include_str!("../input.txt"));
    println!(
        "Part 1: {}",
        root.size_recursive()
            .iter()
            .filter(|s| **s <= 100_000)
            .sum::<usize>()
    );

    const FS_SIZE: usize = 70_000_000;
    const UPDATE_SIZE: usize = 30_000_000;
    let required_freeup = UPDATE_SIZE - (FS_SIZE - root.size());
    println!(
        "Part 2: {}",
        root.size_recursive().iter().fold(usize::MAX, |acc, s| {
            if *s >= required_freeup && *s < acc {
                *s
            } else {
                acc
            }
        })
    );
}

fn parse_file_system(input: &str) -> Directory {
    let mut root = Directory::new("/");
    let mut lines = input.lines().peekable();
    parse_directory(&mut root, &mut lines);
    root
}

fn parse_directory(directory: &mut Directory, lines: &mut Peekable<Lines>) {
    loop {
        if let Some(line) = lines.next() {
            if line.starts_with("$ cd") {
                let dir_name = &line[5..];
                if dir_name != ".." {
                    let sub_dir = directory
                        .directories
                        .iter_mut()
                        .find(|d| d.name == dir_name)
                        .unwrap();
                    parse_directory(sub_dir, lines)
                } else {
                    break;
                }
            } else if line.starts_with("$ ls") {
                loop {
                    if let Some(line) = lines.peek() {
                        if !line.starts_with("$") {
                            let line = lines.next().unwrap();
                            if line.starts_with("dir ") {
                                directory.directories.push(Directory::new(&line[4..]))
                            } else {
                                let mut parts = line.split(" ");
                                let size = parts.next().unwrap().parse::<usize>().unwrap();
                                let _name = String::from(parts.next().unwrap());
                                directory.files.push(File { _name, size })
                            }
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        } else {
            break;
        }
    }
}

struct File {
    _name: String,
    size: usize,
}

struct Directory {
    name: String,
    files: Vec<File>,
    directories: Vec<Directory>,
}

impl Directory {
    fn new(name: &str) -> Self {
        Directory {
            name: String::from(name),
            files: vec![],
            directories: vec![],
        }
    }

    fn size(&self) -> usize {
        self.files.iter().map(|f| f.size).sum::<usize>()
            + self.directories.iter().map(|d| d.size()).sum::<usize>()
    }

    fn size_recursive(&self) -> Vec<usize> {
        let mut sizes = self.directories.iter().fold(vec![], |mut acc, d| {
            acc.append(&mut d.size_recursive());
            acc
        });
        sizes.push(self.size());
        sizes
    }
}

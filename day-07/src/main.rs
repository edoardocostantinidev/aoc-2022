use std::collections::HashMap;

fn main() {
    println!("day 7-1 - {}", day_07_1("./input"));
    println!("day 7-2 - {}", day_07_2("./input"));
}

fn day_07_1(f: &str) -> usize {
    let x = std::fs::read_to_string(f)
        .unwrap()
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|l| l.into())
        .fold(
            Acc {
                scanned_dirs: vec![],
                ..Default::default()
            },
            |acc: Acc, line: Line| match line {
                Line::CommandLine(Command::Ls) => Acc {
                    scanned_dirs: acc.scanned_dirs,
                    last_dir_visited: None,
                    current_dir_being_scanned: acc.last_dir_visited,
                },
                Line::CommandLine(Command::Cd(dir)) => {
                    if dir == "..".to_owned() {
                        Acc
                    } else {
                        if let Acc {
                            current_dir_being_scanned: Some(scanned_dir),
                            ..
                        } = acc
                        {
                            let mut sd = acc.scanned_dirs;
                            sd.push(scanned_dir);
                            Acc {
                                scanned_dirs: sd,
                                current_dir_being_scanned: None,
                                last_dir_visited: Some(Directory {
                                    name: dir,
                                    ..Default::default()
                                }),
                            }
                        } else {
                            Acc {}
                        }
                    }
                }
                Line::OutputLine(Item::File(size, name)) => {}
                Line::OutputLine(Item::Folder(name)) => {}
            },
        );
    dbg!(x);
    0
}

fn day_07_2(f: &str) -> usize {
    todo!()
}

#[derive(Default, Debug)]
struct Acc {
    pub scanned_dirs: Vec<Directory>,
    pub last_dir_visited: Option<Directory>,
    pub current_dir_being_scanned: Option<Directory>,
}

#[derive(Debug, Default, Clone)]
struct Directory {
    pub name: String,
    pub dirs: Vec<Directory>,
    pub files: Vec<File>,
    pub size: usize,
}

#[derive(Debug, Clone)]
struct File {
    pub size: usize,
    pub name: String,
}

#[derive(Debug)]
enum Line {
    CommandLine(Command),
    OutputLine(Item),
}

impl From<&str> for Line {
    fn from(l: &str) -> Self {
        match l.split_at(4) {
            ("$ cd", dir) => Self::CommandLine(Command::Cd(dir.to_owned())),
            ("$ ls", _) => Self::CommandLine(Command::Ls),
            ("dir ", name) => Self::OutputLine(Item::Folder(name.to_owned())),
            (_first, _second) => {
                let elems: Vec<&str> = l.split_ascii_whitespace().collect();
                Self::OutputLine(Item::File(elems[0].parse().unwrap(), elems[1].to_owned()))
            }
        }
    }
}

#[derive(Debug)]
enum Item {
    Folder(String),
    File(usize, String),
}

#[derive(Debug)]
enum Command {
    Cd(String),
    Ls,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_07_1_t() {
        assert_eq!(95437, day_07_1("./test"))
    }
}

use std::collections::HashMap;

fn main() {
    println!("day 7-1 - {}", day_07_1("./input"));
    println!("day 7-2 - {}", day_07_2("./input"));
}

fn day_07_1(f: &str) -> usize {
    let mut current_dir = "/".to_owned();
    let file = std::fs::read_to_string(f).unwrap();
    let lines = file.split('\n');
    let mut sizes = HashMap::<String, usize>::new();
    let mut listing = false;
    let mut last_dir = None;
    for line in lines {
        if line == "" {
            continue;
        }
        if line.starts_with('$') {
            listing = false;
        }

        if !listing {
            match &line[0..4] {
                "$ cd" => {
                    if line == "$ cd ..".to_owned() {
                        current_dir = last_dir.clone().unwrap_or("".to_owned());
                    } else {
                        last_dir = Some(current_dir.clone());
                        let new_dir = line.split_ascii_whitespace().last().unwrap();
                        if new_dir != "/".to_owned() {
                            let new_path = format!("{}{}/", current_dir, new_dir);

                            current_dir = new_path;
                        }
                    }
                }
                "$ ls" => {
                    listing = true;
                }

                _ => {
                    panic!("shouldn't be here")
                }
            }
        } else {
            let x = line.split_ascii_whitespace().collect::<Vec<&str>>()[0];
            match &x[0..3] {
                "dir" => {}
                _ => {
                    let file_size: usize = x.split_ascii_whitespace().collect::<Vec<&str>>()[0]
                        .parse()
                        .unwrap();

                    *sizes.entry(current_dir.to_owned()).or_insert(0) += file_size;
                }
            }
        }
    }

    let keys = sizes
        .clone()
        .keys()
        .filter_map(|s| {
            if sizes[s] <= 100000 {
                return Some(s.clone());
            }
            None
        })
        .collect::<Vec<String>>();

    sizes
        .iter()
        .filter(|x| x.1 <= &100000)
        .map(|x| {
            keys.iter()
                .filter_map(|k| {
                    if x.0.contains(k) {
                        return Some(x.1);
                    }
                    None
                })
                .sum::<usize>()
        })
        .sum()
}

fn day_07_2(f: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_07_1_t() {
        assert_eq!(95437, day_07_1("./test"))
    }
}

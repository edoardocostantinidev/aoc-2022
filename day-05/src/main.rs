use core::panic;
use std::vec;

fn main() {
    println!("day 5-1 - {}", day5_1("./input"));
    println!("day 5-2 - {}", day5_2("./input"));
}

fn day5_1(f: &str) -> String {
    let lines: Vec<String> = std::fs::read_to_string(f)
        .unwrap()
        .split("\n")
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();
    let number_of_stacks: usize = lines
        .iter()
        .filter(|x| x.starts_with(" 1"))
        .map(|l| {
            l.chars()
                .filter(|c| c != &' ')
                .last()
                .unwrap()
                .to_string()
                .parse::<usize>()
                .unwrap()
        })
        .sum();
    let mut stacks: Vec<Vec<char>> = vec![];
    let stacks_s = lines
        .clone()
        .into_iter()
        .take_while(|l| !l.starts_with("m"))
        .filter(|l| l.to_owned() != "".to_owned())
        .filter(|l| !l.starts_with(" 1"))
        .collect::<Vec<String>>();

    let char_at = 1;
    for n in 0..number_of_stacks {
        let s = stacks_s
            .iter()
            .filter_map(|s| match s.as_bytes()[char_at + 4 * n] as char {
                ' ' => None,
                x => Some(x),
            })
            .collect();
        stacks.push(s);
    }

    let moves = lines
        .clone()
        .into_iter()
        .skip_while(|l| !l.starts_with("move"))
        .filter(|l| l.to_owned() != "".to_owned())
        .map(|raw| {
            let slice = raw
                .split(" ")
                .into_iter()
                .skip_while(|w| w == &"move")
                .skip_while(|w| w == &"from")
                .skip_while(|w| w == &"to")
                .map(|n| match n.parse::<i32>() {
                    Ok(x) => x,
                    Err(_) => -1,
                })
                .filter(|x| *x != -1)
                .collect::<Vec<i32>>();
            match slice.as_slice() {
                [quantity, source, destination] => Move {
                    quantity: *quantity,
                    source: *source,
                    destination: *destination,
                },
                _x => {
                    panic!()
                }
            }
        })
        .collect::<Vec<Move>>();

    for move_to_do in moves {
        do_move(&mut stacks, &move_to_do);
    }

    stacks.iter().filter_map(|s| s.get(0)).collect::<String>()
}

fn do_move(stacks: &mut Vec<Vec<char>>, move_to_do: &Move) {
    match move_to_do {
        Move {
            quantity,
            source,
            destination,
        } => {
            for _ in 0..*quantity {
                let el = stacks[(*source as usize) - 1].remove(0);
                let tmp = stacks.get_mut((*destination as usize) - 1).unwrap();
                tmp.reverse();
                tmp.push(el);
                tmp.reverse();
            }
        }
    }
}

fn day5_2(f: &str) -> String {
    let lines: Vec<String> = std::fs::read_to_string(f)
        .unwrap()
        .split("\n")
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();
    let number_of_stacks: usize = lines
        .iter()
        .filter(|x| x.starts_with(" 1"))
        .map(|l| {
            l.chars()
                .filter(|c| c != &' ')
                .last()
                .unwrap()
                .to_string()
                .parse::<usize>()
                .unwrap()
        })
        .sum();
    let mut stacks: Vec<Vec<char>> = vec![];
    let stacks_s = lines
        .clone()
        .into_iter()
        .take_while(|l| !l.starts_with("m"))
        .filter(|l| l.to_owned() != "".to_owned())
        .filter(|l| !l.starts_with(" 1"))
        .collect::<Vec<String>>();

    let char_at = 1;
    for n in 0..number_of_stacks {
        let s = stacks_s
            .iter()
            .filter_map(|s| match s.as_bytes()[char_at + 4 * n] as char {
                ' ' => None,
                x => Some(x),
            })
            .collect();
        stacks.push(s);
    }

    let moves = lines
        .clone()
        .into_iter()
        .skip_while(|l| !l.starts_with("move"))
        .filter(|l| l.to_owned() != "".to_owned())
        .map(|raw| {
            let slice = raw
                .split(" ")
                .into_iter()
                .skip_while(|w| w == &"move")
                .skip_while(|w| w == &"from")
                .skip_while(|w| w == &"to")
                .map(|n| match n.parse::<i32>() {
                    Ok(x) => x,
                    Err(_) => -1,
                })
                .filter(|x| *x != -1)
                .collect::<Vec<i32>>();
            match slice.as_slice() {
                [quantity, source, destination] => Move {
                    quantity: *quantity,
                    source: *source,
                    destination: *destination,
                },
                _x => {
                    panic!()
                }
            }
        })
        .collect::<Vec<Move>>();

    for move_to_do in moves {
        do_move_2(&mut stacks, &move_to_do);
    }

    stacks.iter().filter_map(|s| s.get(0)).collect::<String>()
}

fn do_move_2(stacks: &mut Vec<Vec<char>>, move_to_do: &Move) {
    match move_to_do {
        Move {
            quantity,
            source,
            destination,
        } => {
            let mut buf = vec![];
            for _ in 0..*quantity {
                let el = stacks[(*source as usize) - 1].remove(0);
                buf.push(el);
            }
            let tmp = stacks.get_mut((*destination as usize) - 1).unwrap();
            buf.append(tmp);
            tmp.clear();
            tmp.append(&mut buf);
        }
    }
}

#[derive(Debug)]
pub struct Move {
    pub quantity: i32,
    pub source: i32,
    pub destination: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5_1_t() {
        assert_eq!(String::from("CMZ"), day5_1("./test"))
    }

    #[test]
    fn day5_2_t() {
        assert_eq!(String::from("MCD"), day5_2("./test"))
    }
}

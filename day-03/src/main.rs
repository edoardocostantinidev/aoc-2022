use std::vec;

fn main() {
    println!("day 3-1 - {}", day3_1("./input"));
    println!("day 3-2 - {}", day3_2("./input"));
}

fn day3_2(f: &str) -> i64 {
    std::fs::read_to_string(f)
        .unwrap()
        .split('\n')
        .fold((0, Vec::<char>::default(), 0), |acc, elem| {
            let (sum, common_chars, count) = acc;

            match count {
                0 => {
                    let common_chars = elem.chars().into_iter().collect();
                    (sum, common_chars, count + 1)
                }
                x if x <= 1 => {
                    let common_chars = common_chars
                        .into_iter()
                        .filter(|c| elem.contains(*c))
                        .collect();
                    (sum, common_chars, count + 1)
                }
                2 => {
                    let new_common_chars: Vec<char> = common_chars
                        .into_iter()
                        .filter(|c| elem.contains(*c))
                        .collect();
                    let badge: char = new_common_chars[0];

                    (sum + get_priority(badge), vec![], 0)
                }
                _ => panic!("we should not arrive here"),
            }
        })
        .0
}

fn day3_1(f: &str) -> i64 {
    std::fs::read_to_string(f)
        .unwrap()
        .split('\n')
        .filter_map(|r| {
            let (left, right) = r.split_at(r.len() / 2);
            left.as_bytes()
                .iter()
                .filter(|c| right.contains::<char>(**c as char))
                .last()
        })
        .map(|repeated| get_priority((*repeated).into()))
        .sum()
}

fn get_priority(c: char) -> i64 {
    match c.is_uppercase() {
        true => (c as u8) - 38,
        false => (c as u8) - 96,
    }
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(157, day3_1("./test"))
    }

    #[test]
    fn part_2() {
        assert_eq!(70, day3_2("./test"))
    }
}

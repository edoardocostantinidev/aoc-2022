fn main() {
    println!("day 6-1 - {}", day_6_1("./input"));
    println!("day 6-2 - {}", day_6_2("./input"));
}

fn day_6_2(f: &str) -> usize {
    std::fs::read_to_string(f)
        .unwrap()
        .chars()
        .fold(Buffer::default(), |buf, elem| match buf {
            Buffer {
                inner,
                index: None,
                current_index,
            } => match inner.as_slice() {
                [_one, two, three, four, five, six, seven, eight, nine, ten, eleven, twelve, thirteen,fourteen] if all_different(&[*two, *three, *four, *five, *six, *seven, *eight, *nine, *ten, *eleven, *twelve, *thirteen,*fourteen, elem]) => Buffer {
                    inner: vec![*two, *three, *four, elem],
                    index: Some(current_index + 1),
                    current_index: current_index + 1,
                },
                [_one, two, three, four, five, six, seven, eight, nine, ten, eleven, twelve, thirteen,fourteen] => Buffer {
                    inner: vec![*two, *three, *four, *five, *six, *seven, *eight, *nine, *ten, *eleven, *twelve, *thirteen,*fourteen, elem],
                    index: None,
                    current_index: current_index + 1,
                },
                _ => {
                    let mut new_inner = inner;
                    new_inner.push(elem);
                    Buffer {
                        inner: new_inner,
                        index: None,
                        current_index: current_index + 1,
                    }
                }
            },
            Buffer {
                inner,
                index,
                current_index,
            } => Buffer {
                inner,
                index,
                current_index,
            },
        })
        .index
        .expect("something went wrong!")
}

fn day_6_1(f: &str) -> usize {
    std::fs::read_to_string(f)
        .unwrap()
        .chars()
        .fold(Buffer::default(), |buf, elem| match buf {
            Buffer {
                inner,
                index: None,
                current_index,
            } => match inner.as_slice() {
                [_one, two, three, four] if all_different(&[*two, *three, *four, elem]) => Buffer {
                    inner: vec![*two, *three, *four, elem],
                    index: Some(current_index + 1),
                    current_index: current_index + 1,
                },
                [_one, two, three, four] => Buffer {
                    inner: vec![*two, *three, *four, elem],
                    index: None,
                    current_index: current_index + 1,
                },
                _ => {
                    let mut new_inner = inner;
                    new_inner.push(elem);
                    Buffer {
                        inner: new_inner,
                        index: None,
                        current_index: current_index + 1,
                    }
                }
            },
            Buffer {
                inner,
                index,
                current_index,
            } => Buffer {
                inner,
                index,
                current_index,
            },
        })
        .index
        .expect("something went wrong!")
}

fn all_different(chars: &[char]) -> bool {
    let mut res = true;
    for c in chars {
        res = chars.iter().filter(|ch| c == *ch).count() == 1 && res;
    }
    res
}

#[derive(Default, Debug)]
struct Buffer {
    inner: Vec<char>,
    index: Option<usize>,
    current_index: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_6_1_t() {
        vec![7, 5, 6, 10, 11]
            .iter()
            .enumerate()
            .for_each(|(i, res)| assert_eq!(*res, day_6_1(format!("./test_{i}").as_str())))
    }

    #[test]
    fn day_6_2_t() {
        vec![19, 23, 23, 29, 26]
            .iter()
            .enumerate()
            .for_each(|(i, res)| assert_eq!(*res, day_6_2(format!("./test_{i}").as_str())))
    }
}

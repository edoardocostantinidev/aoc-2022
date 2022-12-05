use std::collections::HashSet;

fn main() {
    println!("day 4-1 - {}", day4_1("./input"));
    println!("day 4-2 - {}", day4_2("./input"));
}

fn day4_1(f: &str) -> i64 {
    std::fs::read_to_string(f)
        .unwrap()
        .split('\n')
        .filter(|s| s != &"")
        .map(|s| {
            let v = s
                .split(',')
                .map(|range| {
                    range
                        .split('-')
                        .filter_map(|n| n.parse::<i64>().ok())
                        .collect::<Vec<i64>>()
                })
                .collect::<Vec<Vec<i64>>>();

            let (first_min, first_max, second_min, second_max) =
                (v[0][0], v[0][1], v[1][0], v[1][1]);

            match (first_min, first_max, second_min, second_max) {
                _ if first_min <= second_min && first_max >= second_max => 1,
                _ if second_min <= first_min && second_max >= first_max => 1,
                _ => 0,
            }
        })
        .sum()
}

fn day4_2(f: &str) -> i64 {
    std::fs::read_to_string(f)
        .unwrap()
        .split('\n')
        .filter(|s| s != &"")
        .map(|s| {
            let v = s
                .split(',')
                .map(|range| {
                    range
                        .split('-')
                        .filter_map(|n| n.parse::<i64>().ok())
                        .collect::<Vec<i64>>()
                })
                .collect::<Vec<Vec<i64>>>();

            let (first_min, first_max, second_min, second_max) =
                (v[0][0], v[0][1], v[1][0], v[1][1]);

            let first_range = (first_min..first_max + 1).collect::<HashSet<i64>>();
            let second_range = (second_min..second_max + 1).collect::<HashSet<i64>>();

            match first_range.intersection(&second_range).count() {
                0 => 0,
                _ => 1,
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_4_1() {
        assert_eq!(2, day4_1("./test"))
    }

    #[test]
    fn day_4_2() {
        assert_eq!(4, day4_2("./test"))
    }
}

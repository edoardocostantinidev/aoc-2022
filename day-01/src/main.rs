fn main() {
    println!("1-1 - {}", max_calories_per_number_of_elves(1));
    println!("1-2 - {}", max_calories_per_number_of_elves(3));
}

fn max_calories_per_number_of_elves(number_of_elves: usize) -> i64 {
    let mut s = std::fs::read_to_string("./input")
        .unwrap()
        .split("\n\n")
        .map(|calories_per_elf| {
            calories_per_elf
                .to_owned()
                .split("\n")
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i64>().unwrap())
                .sum()
        })
        .collect::<Vec<i64>>();

    s.sort();
    s.iter().rev().take(number_of_elves).sum()
}

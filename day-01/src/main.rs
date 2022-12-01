fn main() {
    println!("{}", iterator_approach())
}

fn iterator_approach() -> i64 {
    std::fs::read_to_string("./input")
        .unwrap()
        .split('\n')
        .map(|s| (s.parse::<i64>().unwrap_or(-1), -1))
        .reduce(|(current_count, current_max), (elem, _)| match elem {
            -1 if current_count > current_max => {
                return (0, current_count);
            }
            -1 => (0, current_max),
            _ => (elem + current_count, current_max),
        })
        .unwrap_or_default()
        .1
}

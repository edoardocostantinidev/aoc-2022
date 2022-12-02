use core::panic;

fn main() {
    println!("2-1 - {}", rock_paper_scissor("./input"));
    println!("2-2 - {}", rock_paper_scissor_rigged("./input"));
}

const WIN: i64 = 6;
const DRAW: i64 = 3;

fn rock_paper_scissor(file: &str) -> i64 {
    std::fs::read_to_string(file)
        .unwrap()
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(game_to_points)
        .sum()
}

fn rock_paper_scissor_rigged(file: &str) -> i64 {
    std::fs::read_to_string(file)
        .unwrap()
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|g| game_to_points(rig(g).as_str()))
        .sum()
}

/*

*/
fn rig(game: &str) -> String {
    let pick: char = game.as_bytes()[0].into();
    let expected_result: char = game.as_bytes()[2].into();
    match expected_result {
        'X' => format!("{pick} {}", get_loss(pick)),
        'Y' => format!("{pick} {}", get_draw(pick)),
        'Z' => format!("{pick} {}", get_win(pick)),
        _ => panic!("wtf {game}"),
    }
}

fn get_win(c: char) -> char {
    match c {
        'A' => 'Y',
        'B' => 'Z',
        'C' => 'X',
        _ => panic!("wtf is {c}"),
    }
}

fn get_draw(c: char) -> char {
    match c {
        'A' => 'X',
        'B' => 'Y',
        'C' => 'Z',
        _ => panic!("wtf is {c}"),
    }
}

fn get_loss(c: char) -> char {
    match c {
        'A' => 'Z',
        'B' => 'X',
        'C' => 'Y',
        _ => panic!("wtf is {c}"),
    }
}

/*
X = A = Rock
Y = B = Paper
Z = C = Scissor

wins:
A Y
B Z
C X
losses:
A _
*/

fn game_to_points(game: &str) -> i64 {
    let elf = game.as_bytes()[0].into();
    let me = game.as_bytes()[2].into();
    let my_base_points = get_points(me);
    match (elf, me) {
        ('A', 'Y') | ('B', 'Z') | ('C', 'X') => WIN + my_base_points,
        ('A', 'X') | ('B', 'Y') | ('C', 'Z') => DRAW + my_base_points,
        _ => my_base_points,
    }
}

fn get_points(me: char) -> i64 {
    match me {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        x => panic!("wtf is {x}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(15, rock_paper_scissor("./test"))
    }

    #[test]
    fn test_2() {
        assert_eq!(12, rock_paper_scissor_rigged("./test"))
    }
}

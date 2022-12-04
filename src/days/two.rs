pub struct DayTwo;

use std::collections::HashMap;

use crate::Puzzle;

#[derive(PartialEq, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

fn get_move(theirs: Move, outcome: &str) -> Move {
    match outcome {
        "Z" => match theirs {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        },
        "X" => match theirs {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        },
        "Y" => theirs,
        _ => panic!("Invalid outcome: {}", outcome),
    }
}

fn score(them: Move, me: Move) -> i32 {
    let mut score = match me {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    };
    if them == me {
        score += 3;
    } else if me == Move::Rock && them == Move::Scissors 
        || me == Move::Paper && them == Move::Rock 
        || me == Move::Scissors && them == Move::Paper {
        score += 6;
    }
    println!("{}", score);
    score
}

impl Puzzle for DayTwo {
    fn part1(&self, input: &str) -> String {
        let move_map = HashMap::from([
            ("A", Move::Rock), 
            ("X", Move::Rock), 
            ("B", Move::Paper), 
            ("Y", Move::Paper), 
            ("C", Move::Scissors),
            ("Z", Move::Scissors),
        ]);
        let score = input.lines()
                                      .map(|move_str| {
                                        let moves = move_str.split(' ').collect::<Vec<&str>>();
                                        score(move_map[moves[0]], move_map[moves[1]])
                                      }).sum::<i32>();
        score.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let move_map = HashMap::from([
            ("A", Move::Rock), 
            ("X", Move::Rock), 
            ("B", Move::Paper), 
            ("Y", Move::Paper), 
            ("C", Move::Scissors),
            ("Z", Move::Scissors),
        ]);
        let score = input.lines()
                                      .map(|move_str| {
                                        let moves = move_str.split(' ').collect::<Vec<&str>>();
                                        score(move_map[moves[0]], get_move(move_map[moves[0]], moves[1]))
                                      }).sum::<i32>();
        score.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let puzzle = DayTwo;
        let input = include_str!("../../input/day02.test.txt");
        assert_eq!(puzzle.part1(input), "15");
    }

    #[test]
    fn test_part2() {
        let puzzle = DayTwo;
        let input = include_str!("../../input/day02.test.txt");
        assert_eq!(puzzle.part2(input), "12");
    }
}

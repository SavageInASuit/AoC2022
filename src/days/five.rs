pub struct DayFive;

use crate::Puzzle;


fn parse_input(input: &str) -> Vec<Vec<&str>> {
    let mut columns: Vec<Vec<&str>> = Vec::new();

    for row in input.lines() {
        let mut offset = 0;
        while offset < row.len() {
            let sub = &row[offset..(offset+3)];
            if sub == "   " || sub.len() < 3 {
                offset += 4;
                continue;
            }
            let i = offset / 4;
            while i >= columns.len() {
                columns.push(Vec::new());
            }
            let column = &mut columns[i];
            column.push(&sub[1..2]);
            offset += 4;
        }
        if row == "" {
            break;
        }
    }
    for column in columns.iter_mut() {
        column.reverse();
    }

    columns
}

fn top_letters<'a>(columns: &'a Vec<Vec<&'a str>>) -> Vec<&str> {
    columns.iter().map(|column| *column.last().unwrap()).collect()
}

impl Puzzle for DayFive {
    fn part1(&self, input: &str) -> String {
        let mut columns = parse_input(input);
        let parts: Vec<&str> = input.split("\n\n").collect();
        let instructions = parts[1];
        for line in instructions.lines() {
            let line = &line[5..];
            let parts: Vec<&str> = line.split(" from ").collect();
            let amount: usize = parts[0].parse().unwrap();
            let parts: Vec<&str> = parts[1].split(" to ").collect();
            let from: usize = parts[0].parse().unwrap();
            let to: usize = parts[1].parse().unwrap();

            for _ in 0..amount {
                let to_move = columns[from - 1].pop().unwrap();
                columns[to - 1].push(to_move);
            }
        }
        let letters = top_letters(&columns);
        letters.join("")
    }

    fn part2(&self, input: &str) -> String {
        let mut columns = parse_input(input);
        let parts: Vec<&str> = input.split("\n\n").collect();
        let instructions = parts[1];
        for line in instructions.lines() {
            let line = &line[5..];
            let parts: Vec<&str> = line.split(" from ").collect();
            let amount: usize = parts[0].parse().unwrap();
            let parts: Vec<&str> = parts[1].split(" to ").collect();
            let from: usize = parts[0].parse().unwrap();
            let to: usize = parts[1].parse().unwrap();

            let column = &mut columns[from - 1];
            let mut items: Vec<&str> = column.drain((column.len() - amount)..).collect();
            let to_column = &mut columns[to - 1];
            to_column.append(&mut items);
        }
        let letters = top_letters(&columns);
        letters.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let puzzle = DayFive;
        let input = include_str!("../../input/day05.test.txt");
        assert_eq!(puzzle.part1(input), "CMZ");
    }

    #[test]
    fn test_part2() {
        let puzzle = DayFive;
        let input = include_str!("../../input/day05.test.txt");
        assert_eq!(puzzle.part2(input), "MCD");
    }
}

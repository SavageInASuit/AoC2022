pub struct DayEleven;

use crate::Puzzle;

#[derive(Debug)]
struct Monkey {
    id: u32,
    items: Vec<u32>,
    operation: fn() -> u32,
    test: fn() -> u32,
}

impl Monkey {
    fn from(s: String) -> Self {
        let parts: Vec<&str> = s.split('\n').collect();
        let id_parts = parts[0].split(' ').collect::<Vec<&str>>();
        let id = id_parts[1][..(id_parts[1].len() - 1)].parse::<u32>().unwrap();
        let items = parts[1].split(": ").collect::<Vec<&str>>()[1].split(", ").map(|item| item.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        Monkey {
            id,
            items,
            operation: || 0,
            test: || 0,
        }
    }
}

impl Puzzle for DayEleven {
    fn part1(&self, input: &str) -> String {
        let mut monkeys = input.split("\n\n").map(|line| Monkey::from(line.to_string())).collect::<Vec<Monkey>>();
        println!("{:?}", monkeys);
        "".to_owned()
    }

    fn part2(&self, input: &str) -> String {
        "".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let puzzle = DayEleven;
        let input = include_str!("../../input/day11.test.txt");
        assert_eq!(puzzle.part1(input), "13140");
    }

    #[test]
    fn test_part2() {
        let puzzle = DayEleven;
        let input = include_str!("../../input/day11.test.txt");
        assert_eq!(puzzle.part2(input), "111");
    }
}

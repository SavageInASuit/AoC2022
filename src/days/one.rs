pub struct DayOne;

use crate::Puzzle;

impl Puzzle for DayOne {
    fn part1(&self, input: &str) -> String {
        let most_calories = input.split("\n\n")
                                      .map(|elf| {
                                          elf.lines().map(|calories| {
                                              calories.parse::<i32>().unwrap()
                                          }).sum::<i32>()
                                      }).max().unwrap();
        most_calories.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut elves = input.split("\n\n")
                                      .map(|elf| {
                                          elf.lines().map(|calories| {
                                              calories.parse::<i32>().unwrap()
                                          }).sum::<i32>()
                                      }).collect::<Vec<i32>>();
        elves.sort_by(|a, b| b.cmp(a));
        format!("{}", elves[0] + elves[1] + elves[2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let puzzle = DayOne;
        let input = include_str!("../../input/day01.test.txt");
        assert_eq!(puzzle.part1(input), "24000");
    }

    #[test]
    fn test_part2() {
        let puzzle = DayOne;
        let input = include_str!("../../input/day01.test.txt");
        assert_eq!(puzzle.part2(input), "45000");
    }
}
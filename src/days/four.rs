pub struct DayFour;

use crate::Puzzle;

impl Puzzle for DayFour {
    fn part1(&self, input: &str) -> String {
        let containments: u32 = input.lines()
            .map(|line| {
                let mut split = line.split(',');
                let first = split.next().unwrap().split('-').map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
                let second = split.next().unwrap().split('-').map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
                if (first[0] <= second[0] && first[1] >= second[1]) 
                    || (second[0] <= first[0] && second[1] >= first[1]) {
                    1
                } else {
                    0
                }
            }).sum();

        containments.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let containments: u32 = input.lines()
            .map(|line| {
                let mut split = line.split(',');
                let first = split.next().unwrap().split('-').map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
                let second = split.next().unwrap().split('-').map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
                if (first[0] <= second[0] && first[1] >= second[0]) 
                    || (first[0] <= second[1] && first[1] >= second[1]) 
                    || (second[0] <= first[0] && second[1] >= first[0]) 
                    || (second[0] <= first[1] && second[1] >= first[1]) 
                {
                    1
                } else {
                    0
                }
            }).sum();

        containments.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let puzzle = DayFour;
        let input = include_str!("../../input/day04.test.txt");
        assert_eq!(puzzle.part1(input), "2");
    }

    #[test]
    fn test_part2() {
        let puzzle = DayFour;
        let input = include_str!("../../input/day04.test.txt");
        assert_eq!(puzzle.part2(input), "4");
    }
}

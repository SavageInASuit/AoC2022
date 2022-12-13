pub struct DayThree;

use crate::Puzzle;

fn score_char(c: &char) -> u32 {
    let code = *c as u32;
    if code >= 0x61 {
        code - 0x60
    } else { // char is A-Z - 27 to 52 
        code - 0x26
    }
}

impl Puzzle for DayThree {
    fn part1(&self, input: &str) -> String {
        let sum: u32 = input.lines()
            .map(|line| {
                let compartments = line.split_at(line.len() / 2);
                // check which chars exist in both compartments
                let chars: Vec<char> = compartments.0.chars()
                    .filter(|c| compartments.1.contains(*c))
                    .collect::<Vec<char>>();
                let char = chars.get(0).unwrap();
                // get character code for the duplicated item
                score_char(char)
            }).sum();
        
        sum.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let elves: Vec<&str> = input.lines().collect();
        let mut score = 0;
        for group in elves.chunks(3) {
            // find character that exists in all three strings
            let chars: Vec<char> = group[0].chars()
                .filter(|c| group[1].contains(*c) && group[2].contains(*c))
                .collect::<Vec<char>>();
            score += score_char(chars.get(0).unwrap());
        }
        score.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_score() {
        assert_eq!(score_char(&'p'), 16, "Character {}", 'p');
        assert_eq!(score_char(&'L'), 38, "Character {}", 'L');
        assert_eq!(score_char(&'P'), 42, "Character {}", 'P');
        assert_eq!(score_char(&'v'), 22, "Character {}", 'v');
        assert_eq!(score_char(&'t'), 20, "Character {}", 't');
        assert_eq!(score_char(&'s'), 19, "Character {}", 's');
    }

    #[test]
    fn test_part1() {
        let puzzle = DayThree;
        let input = include_str!("../../input/day03.test.txt");
        assert_eq!(puzzle.part1(input), "157");
    }

    #[test]
    fn test_part2() {
        let puzzle = DayThree;
        let input = include_str!("../../input/day03.test.txt");
        assert_eq!(puzzle.part2(input), "70");
    }
}

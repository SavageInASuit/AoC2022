pub struct DaySix;

use std::{collections::{HashMap, VecDeque}, hash::Hash};

use crate::Puzzle;

fn vec_unique<T>(v: &Vec<T>) -> bool 
where 
    T: Eq + Hash
{
    let mut map: HashMap<&T, bool> = HashMap::new();
    for t in v {
        if map.get(&t).is_some() {
            return false;
        }
        map.insert(t, true);
    }
    true
}

impl Puzzle for DaySix {
    fn part1(&self, input: &str) -> String {
        // Use a sliding window to find end index of the first four unique characters in the input
        let mut chars = input.chars();
        let mut unique_chars = chars.by_ref().take(4).collect::<VecDeque<char>>();
        let mut ind = 4;
        while !vec_unique(&unique_chars.iter().collect::<Vec<&char>>()) {
            unique_chars.pop_front();
            unique_chars.push_back(chars.next().unwrap());
            ind += 1;
        }
        
        ind.to_string()
    }

    fn part2(&self, input: &str) -> String {
        // Use a sliding window to find end index of the first fourteen unique characters in the input
        let mut chars = input.chars();
        let mut unique_chars = chars.by_ref().take(14).collect::<VecDeque<char>>();
        let mut ind = 14;
        while !vec_unique(&unique_chars.iter().collect::<Vec<&char>>()) {
            unique_chars.pop_front();
            unique_chars.push_back(chars.next().unwrap());
            ind += 1;
        }
        
        ind.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_unique() {
        assert!(vec_unique(&vec![1, 2, 3]));
        assert!(!vec_unique(&vec![1, 1, 3]));
        assert!(vec_unique(&vec!["a", "b", "c"]));
        assert!(!vec_unique(&vec!["a", "b", "a"]));
        assert!(vec_unique(&vec![1, 5555555, 555555, 4]));
        assert!(!vec_unique(&vec![1, 5555555, 555555, 4, 5555555]));
    }

    #[test]
    fn test_part1() {
        let puzzle = DaySix;
        let mut input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(puzzle.part1(input), "7");
        input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(puzzle.part1(input), "5");
        input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(puzzle.part1(input), "6");
        input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(puzzle.part1(input), "10");
        input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(puzzle.part1(input), "11");
    }

    #[test]
    fn test_part2() {
        let puzzle = DaySix;
        let mut input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(puzzle.part2(input), "19");
        input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(puzzle.part2(input), "23");
        input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(puzzle.part2(input), "23");
        input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(puzzle.part2(input), "29");
        input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(puzzle.part2(input), "26");
    }
}

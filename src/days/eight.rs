pub struct DayEight;

use crate::Puzzle;

fn is_visible(forest: &Vec<Vec<u32>>, x: &usize, y: &usize) -> bool {
    let this_size = forest[*y][*x];
    // Check up from position
    let mut visible = true;
    for dx in 0..forest[*y].len() {
        if dx == *x {
            if visible {
                return true;
            }
            visible = true;
        }
        if dx != *x && forest[*y][dx] >= this_size {
            visible = false;
        }
    }
    if visible {
        return true;
    }
    visible = true;
    for dy in 0..forest.len() {
        if dy == *y {
            if visible {
                return true;
            }
            visible = true;
        }

        if dy != *y && forest[dy][*x] >= this_size {
            visible = false;
        }
    }
    visible
}

fn scenic_score(forest: &Vec<Vec<u32>>, x: &usize, y: &usize) -> u32 {
    let this_size = forest[*y][*x];
    // scenic score is the product of the number of trees visible from up, down, left, and right
    // Check up from position
    let mut up_trees = 0;
    for dy in (0..*y).rev() {
        up_trees += 1;
        if forest[dy][*x] >= this_size {
            break;
        }
    }
    let mut down_trees = 0;
    for dy in (*y + 1)..forest.len() {
        down_trees += 1;
        if forest[dy][*x] >= this_size {
            break;
        }
    }
    let mut left_trees = 0;
    for dx in (0..*x).rev() {
        left_trees += 1;
        if forest[*y][dx] >= this_size {
            break;
        }
    }
    let mut right_trees = 0;
    for dx in (*x + 1)..forest[*y].len() {
        right_trees += 1;
        if forest[*y][dx] >= this_size {
            break;
        }
    }
    up_trees * down_trees * left_trees * right_trees
}

impl Puzzle for DayEight {
    fn part1(&self, input: &str) -> String {
        let forest: Vec<Vec<u32>> = input.lines().map(|line| line.chars().map(|height| (height.to_digit(10).unwrap() + 1)).collect()).collect();

        // Iterate over each tree and check if it is visible from up, down, left, or right
        let mut visible_trees: u32 = 0;
        for y in 0..forest.len() {
            for x in 0..forest[0].len() {
                if is_visible(&forest, &x, &y) {
                    visible_trees += 1;
                }
            }
        }
        
        visible_trees.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let forest: Vec<Vec<u32>> = input.lines().map(|line| line.chars().map(|height| (height.to_digit(10).unwrap() + 1)).collect()).collect();

        // Iterate over each tree and check if it is visible from up, down, left, or right
        let mut highest_scenic: u32 = 0;
        for y in 0..forest.len() {
            for x in 0..forest[0].len() {
                let score = scenic_score(&forest, &x, &y);
                if score > highest_scenic {
                    highest_scenic = score;
                }
            }
        }

        highest_scenic.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visible() {
        let forest = vec![
            vec![1, 5, 6, 3],
            vec![5, 5, 5, 6],
            vec![7, 8, 4, 9],
        ];
        assert!(is_visible(&forest, &0, &0));
        assert!(is_visible(&forest, &0, &1));
        assert!(is_visible(&forest, &0, &2));

        assert!(is_visible(&forest, &3, &0));
        assert!(is_visible(&forest, &3, &1));
        assert!(is_visible(&forest, &2, &2));

        assert!(is_visible(&forest, &1, &0));
        assert!(is_visible(&forest, &1, &2));

        assert!(!is_visible(&forest, &1, &1));
        assert!(is_visible(&forest, &2, &1));
    }

    #[test]
    fn test_part1() {
        let puzzle = DayEight;
        let input = include_str!("../../input/day08.test.txt");
        assert_eq!(puzzle.part1(input), "21");
    }

    #[test]
    fn test_part2() {
        let puzzle = DayEight;
        let input = include_str!("../../input/day08.test.txt");
        assert_eq!(puzzle.part2(input), "8");
    }
}

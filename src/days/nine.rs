pub struct DayNine;

use std::collections::HashSet;

use crate::Puzzle;

struct Move {
    x: i32,
    y: i32,
}
impl Move {
    fn from(s: String) -> Self {
        let parts = s.split(' ').collect::<Vec<&str>>();
        let dir = parts[0];
        let dist = parts[1].parse::<i32>().unwrap();
        match dir {
            "U" => Move { x: 0, y: dist },
            "D" => Move { x: 0, y: -dist },
            "L" => Move { x: -dist, y: 0 },
            "R" => Move { x: dist, y: 0 },
            _ => panic!("Unknown direction: {}", dir),
        }
    }
}

struct Rope {
    segments: Vec<[i32; 2]>,
    travelled_to: HashSet<[i32; 2]>,
}

impl Rope {
    fn new(seg_count: u8) -> Self {
        let mut segments = Vec::new();
        for _ in 0..seg_count {
            segments.push([0, 0]);
        }
        Rope {
            segments,
            travelled_to: HashSet::new(),
        }
    }

    fn pull_tail(&mut self) {
        for i in 1..self.segments.len() {
            let dx = self.segments[i - 1][0] - self.segments[i][0];
            let dy = self.segments[i - 1][1] - self.segments[i][1];
            if dx > 1 && dy == 0 {
                self.segments[i][0] += 1;
            } else if dx < -1 && dy == 0 {
                self.segments[i][0] -= 1;
            } else if dx == 0 && dy > 1 {
                self.segments[i][1] += 1;
            } else if dx == 0 && dy < -1 {
                self.segments[i][1] -= 1;
            } else if dx.abs() > 1 || dy.abs() > 1 {
                self.segments[i][0] += if dx > 0 { 1 } else { -1 };
                self.segments[i][1] += if dy > 0 { 1 } else { -1 };
            }   
        }
        

        self.travelled_to.insert([self.segments[self.segments.len() - 1][0], self.segments[self.segments.len() - 1][1]]);
    }

    fn apply_move(&mut self, m: &Move) {
        let ind = if m.x != 0 { 0 } else { 1 };
        let dist = if m.x != 0 { m.x } else { m.y };
        for _ in 0..dist.abs() {
            if dist > 0 {
                self.segments[0][ind] += 1;
            } else {
                self.segments[0][ind] -= 1;
            }
            self.pull_tail();
        }
    }
}

impl Puzzle for DayNine {
    fn part1(&self, input: &str) -> String {
        let mut rope = Rope::new(2);
        let moves = input.lines().map(|l| Move::from(l.to_owned())).collect::<Vec<Move>>();

        for rope_move in moves.iter() {
            rope.apply_move(rope_move);
        }

        rope.travelled_to.len().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut rope = Rope::new(10);
        let moves = input.lines().map(|l| Move::from(l.to_owned())).collect::<Vec<Move>>();

        for rope_move in moves.iter() {
            rope.apply_move(rope_move);
        }

        rope.travelled_to.len().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let puzzle = DayNine;
        let input = include_str!("../../input/day09.test.txt");
        assert_eq!(puzzle.part1(input), "13");
    }

    #[test]
    fn test_part2() {
        let puzzle = DayNine;
        let input = include_str!("../../input/day09.test2.txt");
        assert_eq!(puzzle.part2(input), "36");
    }
}

pub struct DayTen;

use crate::Puzzle;

enum Cmd {
    Noop,
    Addx(i32),
}

impl Cmd {
    fn from(s: String) -> Self {
        let parts = s.split(' ').collect::<Vec<&str>>();
        if parts.len() == 1 {
            Cmd::Noop
        } else {
            let val = parts[1].parse::<i32>().unwrap();
            Cmd::Addx(val)
        }
    }
}

impl Puzzle for DayTen {
    fn part1(&self, input: &str) -> String {
        let mut reg_x = 1;
        let mut signal_strength = 0;
        let mut cycle = 1;
        for line in input.lines() {
            let cmd = Cmd::from(line.to_owned());
            match cmd {
                Cmd::Addx(val) => {
                    if (cycle - 20) % 40 == 0 {
                        signal_strength += reg_x * cycle;
                    }
                    if (cycle - 19) % 40 == 0 {
                        signal_strength += reg_x * (cycle + 1);
                    }
                    reg_x += val;
                    cycle += 2;
                },
                Cmd::Noop => {
                    if (cycle - 20) % 40 == 0 {
                        signal_strength += reg_x * cycle;
                    }
                    cycle += 1;
                },
            }
        };
        signal_strength.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut reg_x = 1;
        let mut cycle = 1;
        let mut screen: String = "".to_owned();
        for line in input.lines() {
            let cmd = Cmd::from(line.to_owned());
            let check = cycle == 41;
            match cmd {
                Cmd::Addx(val) => {
                    let pos = (cycle - 1) % 40;
                    let next = cycle % 40;
                    if pos == reg_x - 1 || pos == reg_x || pos == reg_x + 1 {
                        screen.push('#');
                    } else {
                        screen.push('.');
                    }
                    if cycle % 40 == 0 {
                        screen.push('\n');
                    }
                    if next == reg_x - 1 || next == reg_x || next == reg_x + 1 {
                        screen.push('#');
                    } else {
                        screen.push('.');
                    }
                    if (cycle + 1) % 40 == 0 {
                        screen.push('\n');
                    }

                    reg_x += val;
                    cycle += 2;
                },
                Cmd::Noop => {
                    let pos = (cycle - 1) % 40;
                    if pos == reg_x - 1 || pos == reg_x || pos == reg_x + 1 {
                        screen.push('#');
                    } else {
                        screen.push('.');
                    }
                    if cycle % 40 == 0 {
                        screen.push('\n');
                    }
                    cycle += 1;
                },
            }
        };
        screen
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let puzzle = DayTen;
        let input = include_str!("../../input/day10.test.txt");
        assert_eq!(puzzle.part1(input), "13140");
    }

    #[test]
    fn test_part2() {
        let puzzle = DayTen;
        let input = include_str!("../../input/day10.test.txt");
        assert_eq!(puzzle.part2(input), "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
");
    }
}

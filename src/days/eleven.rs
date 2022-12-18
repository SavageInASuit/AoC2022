pub struct DayEleven;

use std::{collections::HashMap, ops::IndexMut};

use crate::Puzzle;

struct Monkey {
    items: Vec<u128>,
    inspections: u32,
    operation: Vec<String>,
    test: u128,
    true_monkey: u32,
    false_monkey: u32,
}

impl Monkey {
    fn from(s: String) -> Self {
        let parts: Vec<&str> = s.split('\n').collect();
        let items = parts[1].split(": ").collect::<Vec<&str>>()[1]
            .split(", ")
            .map(|item| item.parse::<u128>().unwrap())
            .collect::<Vec<u128>>();
        let operation = parts[2]
            .trim()
            .split(' ')
            .map(|s| s.to_owned())
            .collect::<Vec<String>>()
            .split_off(3);
        let div_by = parts[3].trim().split(' ').collect::<Vec<&str>>()[3];
        let test = div_by.parse::<u128>().unwrap();
        let true_monkey = parts[4].trim().split(' ').collect::<Vec<&str>>()[5]
            .parse::<u32>()
            .unwrap();
        let false_monkey = parts[5].trim().split(' ').collect::<Vec<&str>>()[5]
            .parse::<u32>()
            .unwrap();
        Monkey {
            items,
            inspections: 0,
            operation,
            test,
            true_monkey,
            false_monkey,
        }
    }

    fn compute_operation(&self, old: u128) -> u128 {
        let operator = &self.operation[1];
        let right = &self.operation[2];
        let right_val = if right == "old" {
            old
        } else {
            right.parse::<u128>().unwrap()
        };

        match operator.as_str() {
            "+" => old + right_val,
            "-" => old - right_val,
            "*" => old * right_val,
            "/" => old / right_val,
            _ => panic!("Unknown operator {}", operator),
        }
    }
}

fn get_common_multiple(monkeys: &Vec<Monkey>) -> u128 {
    let mut lcm = 1;
    for monkey in monkeys {
        lcm *= monkey.test;
    }
    lcm
}

impl Puzzle for DayEleven {
    fn part1(&self, input: &str) -> String {
        let mut monkeys = input
            .split("\n\n")
            .map(|line| Monkey::from(line.to_string()))
            .collect::<Vec<Monkey>>();
        for _ in 0..20 {
            for i in 0..monkeys.len() {
                let monkey = monkeys.index_mut(i);
                let mut passes: HashMap<u32, Vec<u128>> = HashMap::new();
                while !&monkey.items.is_empty() {
                    monkey.inspections += 1;
                    let item = monkey.items.pop().unwrap();
                    let mut new_val = monkey.compute_operation(item);
                    new_val /= 3;
                    if new_val % monkey.test == 0 {
                        // pass to true monkey
                        let id = monkey.true_monkey;
                        match passes.get_mut(&id) {
                            Some(items) => items.push(new_val),
                            None => {
                                passes.insert(id, vec![new_val]);
                            }
                        }
                    } else {
                        // pass to false monkey
                        let id = monkey.false_monkey;
                        match passes.get_mut(&id) {
                            Some(items) => items.push(new_val),
                            None => {
                                passes.insert(id, vec![new_val]);
                            }
                        }
                    }
                }
                // loop through passes and add to monkeys
                for (id, items) in passes {
                    let monkey = monkeys.get_mut(id as usize).unwrap();
                    monkey.items.extend(items);
                }
            }
        }
        // find the two monkeys with the most inspections
        let mut max = 0;
        let mut second_max = 0;
        for monkey in monkeys {
            if monkey.inspections > max {
                second_max = max;
                max = monkey.inspections;
            } else if monkey.inspections > second_max {
                second_max = monkey.inspections;
            }
        }
        (max * second_max).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut monkeys = input
            .split("\n\n")
            .map(|line| Monkey::from(line.to_string()))
            .collect::<Vec<Monkey>>();
        // Used to prevent overflow
        let lcm = get_common_multiple(&monkeys);
        for _ in 0..10000 {
            for i in 0..monkeys.len() {
                let monkey = monkeys.index_mut(i);
                let mut passes: HashMap<u32, Vec<u128>> = HashMap::new();
                while !&monkey.items.is_empty() {
                    monkey.inspections += 1;
                    let item = monkey.items.pop().unwrap();
                    let new_val = monkey.compute_operation(item) % lcm;
                    if new_val % monkey.test == 0 {
                        // pass to true monkey
                        let id = monkey.true_monkey;
                        match passes.get_mut(&id) {
                            Some(items) => items.push(new_val),
                            None => {
                                passes.insert(id, vec![new_val]);
                            }
                        }
                    } else {
                        // pass to false monkey
                        let id = monkey.false_monkey;
                        match passes.get_mut(&id) {
                            Some(items) => items.push(new_val),
                            None => {
                                passes.insert(id, vec![new_val]);
                            }
                        }
                    }
                }
                // loop through passes and add to monkeys
                for (id, items) in passes {
                    let monkey = monkeys.get_mut(id as usize).unwrap();
                    monkey.items.extend(items);
                }
            }
        }
        // find the two monkeys with the most inspections
        let mut max = 0;
        let mut second_max = 0;
        for monkey in monkeys {
            if monkey.inspections > max {
                second_max = max;
                max = monkey.inspections;
            } else if monkey.inspections > second_max {
                second_max = monkey.inspections;
            }
        }
        // ...we have to pull the two numbers here, as the multiplication overflow!
        println!("Max: {}, Second Max: {}", max, second_max);
        (max * second_max).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let puzzle = DayEleven;
        let input = include_str!("../../input/day11.test.txt");
        assert_eq!(puzzle.part1(input), "10605");
    }

    #[test]
    fn test_part2() {
        let puzzle = DayEleven;
        let input = include_str!("../../input/day11.test.txt");
        assert_eq!(puzzle.part2(input), "2713310158");
    }
}

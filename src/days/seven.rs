pub struct DaySeven;

use std::collections::HashMap;

use crate::Puzzle;

#[derive(PartialEq)]
enum States {
    Cmd,
    List,
}

fn ancestors(path: &mut Vec<&str>) -> Vec<String> {
    let mut an: Vec<String> = Vec::new();

    for i in 0..path.len() {
        let path_str = path[0..(path.len() - i)].join("/");
        an.push(path_str.to_owned());
    }

    an
}

impl Puzzle for DaySeven {
    fn part1(&self, input: &str) -> String {
        let mut state = States::Cmd;
        let mut dir: Vec<&str> = vec![];
        let limit = 100_000;
        let mut sizes: HashMap<String, u32> = HashMap::new();
        for line in input.lines() {
            if state == States::List && !line.starts_with('$') {
                // another file or dir to enumerate
                let parts: Vec<&str> = line.split(' ').collect();
                if parts[0] != "dir" {
                    let size = parts[0].parse().expect("file size not an integer!");
                    for ancestor in ancestors(&mut dir).iter() {
                        let cur_size = sizes.get(&ancestor.clone()[..]);
                        match cur_size {
                            Some(s) => sizes.insert(ancestor.clone(), s + size),
                            None => sizes.insert(ancestor.clone(), size),
                        };
                    }
                }
            } else if line.starts_with('$') {
                let parts: Vec<&str> = line.split(' ').collect();
                match parts[1] {
                    "cd" => {
                        state = States::Cmd;
                        if parts[2] == ".." {
                            dir.pop();
                        } else {
                            dir.push(parts[2])
                        }
                    },
                    "ls" => state = States::List,
                    _ => panic!("Unexpected command {}", parts[1]),
                };
            }
        }
        println!("Sizes: {:?}", sizes);
        sizes.into_values().filter(|v| v <= &limit).sum::<u32>().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut state = States::Cmd;
        let mut dir: Vec<&str> = vec![];
        let mut sizes: HashMap<String, u32> = HashMap::new();
        for line in input.lines() {
            if state == States::List && !line.starts_with('$') {
                // another file or dir to enumerate
                let parts: Vec<&str> = line.split(' ').collect();
                if parts[0] != "dir" {
                    let size = parts[0].parse().expect("file size not an integer!");
                    for ancestor in ancestors(&mut dir).iter() {
                        let cur_size = sizes.get(&ancestor.clone()[..]);
                        match cur_size {
                            Some(s) => sizes.insert(ancestor.clone(), s + size),
                            None => sizes.insert(ancestor.clone(), size),
                        };
                    }
                }
            } else if line.starts_with('$') {
                let parts: Vec<&str> = line.split(' ').collect();
                match parts[1] {
                    "cd" => {
                        state = States::Cmd;
                        if parts[2] == ".." {
                            dir.pop();
                        } else {
                            dir.push(parts[2])
                        }
                    },
                    "ls" => state = States::List,
                    _ => panic!("Unexpected command {}", parts[1]),
                };
            }
        }
        let hd_size = 70_000_000;
        let needed = 30_000_000;
        let used = sizes.get("/").expect("no root dir found");
        let to_free = needed - (hd_size - used);
        // find the smallest dir that is larger than to_free
        let mut smallest = u32::MAX;
        for (_, size) in sizes.iter() {
            if size > &to_free && size < &smallest {
                smallest = *size;
            }
        }
        smallest.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let puzzle = DaySeven;
        let input = include_str!("../../input/day07.test.txt");
        assert_eq!(puzzle.part1(input), "95437");
    }

    #[test]
    fn test_part2() {
        let puzzle = DaySeven;
        let input = include_str!("../../input/day07.test.txt");
        assert_eq!(puzzle.part2(input), "24933642");
    }
}

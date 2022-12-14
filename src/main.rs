use std::path::PathBuf;
use std::env;
use std::fs;
use std::io;

pub mod days;

// A trait for each day's puzzle
pub(crate) trait Puzzle {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
}

// input reader utility function
fn read_input(day: u8, test: bool) -> io::Result<String> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("input");
    let mut file_name = format!("day{:02}.txt", day);
    if test {
        file_name = format!("day{:02}.test.txt", day);
    }

    path.push(&file_name);

    let file_result = fs::read_to_string(path);
    match file_result {
        Err(e) => {
            Err(io::Error::new(io::ErrorKind::NotFound, format!("Error reading input file: {} - {}", e, file_name)))
        },
        _ => file_result,
    }
}

// get the puzzle for the given day
fn get_puzzle(day: u8) -> Box<dyn Puzzle> {
    match day {
        1 => Box::new(days::one::DayOne),
        2 => Box::new(days::two::DayTwo),
        3 => Box::new(days::three::DayThree),
        4 => Box::new(days::four::DayFour),
        5 => Box::new(days::five::DayFive),
        6 => Box::new(days::six::DaySix),
        7 => Box::new(days::seven::DaySeven),
        8 => Box::new(days::eight::DayEight),
        9 => Box::new(days::nine::DayNine),
        10 => Box::new(days::ten::DayTen),
        11 => Box::new(days::eleven::DayEleven),
        _ => panic!("Day {} not implemented", day),
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let day = args[1].parse::<u8>().unwrap();
    let mut test = false;
    if args.len() > 2 {
        test = args[2] == "test";
    }

    let data = read_input(day, test);

    match data {
        Ok(input) => {
            let puzzle = get_puzzle(day);
            println!("{}", puzzle.part1(&input));
            println!("{}", puzzle.part2(&input));
        }
        Err(e) => println!("{}", e),
    }
}

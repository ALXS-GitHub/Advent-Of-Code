use core::panic;
use std::io::{self, Read};
use std::fs::{self, File};
use std::env;
use dotenv::dotenv;
use reqwest::blocking::Client;

pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_test_part1() {
        let correct_answer = 0;
        let input: Vec<String> = read_input("inputtest.txt");
        let answer = part1::part1(&input);
        assert_eq!(answer, correct_answer, "\x1b[31m\x1b[1mExpected {}, got {}\x1b[0m", correct_answer, answer);
    }

    #[test]
    fn test_part1() {
        let correct_answer = 115;
        let input: Vec<String> = read_input("input.txt");
        let answer = part1::part1(&input);
        assert_eq!(answer, correct_answer, "\x1b[31m\x1b[1mExpected {}, got {}\x1b[0m", correct_answer, answer);
    }

    #[test]
    #[ignore]
    fn test_test_part2() {
        let correct_answer = 0;
        let input: Vec<String> = read_input("inputtest.txt");
        let answer = part2::part2(&input);
        assert_eq!(answer, correct_answer, "\x1b[31m\x1b[1mExpected {}, got {}\x1b[0m", correct_answer, answer);
    }

    #[test]
    fn test_part2() {
        let correct_answer = 221;
        let input: Vec<String> = read_input("input.txt");
        let answer = part2::part2(&input);
        assert_eq!(answer, correct_answer, "\x1b[31m\x1b[1mExpected {}, got {}\x1b[0m", correct_answer, answer);
    }
}

pub fn read_input(path: &str) -> Vec<String> {
    let mut input = String::new();
    match File::open(path) {
        Ok(mut file) => {
            file.read_to_string(&mut input).unwrap();
        }
        Err(_) => {
            panic!("Failed to open file '{}'", path);
        }
    }
    let input = input.lines().map(|s| s.to_string()).collect();

    return input;
}

pub fn fetch_input(path: &str, year: u32, day: u32, force: bool) {

    if !force && fs::metadata(path).is_ok() {
        return;
    }

    dotenv().ok();
    let session_token = env::var("SESSION_TOKEN").expect("SESSION_TOKEN not set");

    let client = Client::new();
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    let response = client
        .get(url)
        .header("cookie", format!("session={}", session_token))
        .send()
        .expect("Failed to fetch input")
        .text()
        .expect("Failed to read response text");

    let response = response.trim_end().to_string();

    fs::write(path, response).expect("Failed to write input file");

    println!("\x1b[33m\x1b[1mFetched input for day {} of year {} to '{}'\x1b[0m", day, year, path);
}

pub fn get_day() -> u32 {
    let d = "day12".to_string()
        .trim_start_matches("day")
        .parse::<u32>()
        .expect("Invalid day number");
    d
}

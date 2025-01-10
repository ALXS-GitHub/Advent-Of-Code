use day7::{read_input, fetch_input, get_day, part1, part2};
use std::env;
use std::time::{Instant};

fn main() {

    // Config
    let path = "input.txt";
    let test_path = "inputtest.txt"; // no methods to fetch test input, so it must be provided manually
    let year: u32 = 2016;
    let day: u32 = get_day();

    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    let part1 = args.contains(&"--part1".to_string());
    let part2 = args.contains(&"--part2".to_string());
    let fetch = args.contains(&"--fetch".to_string());
    let test = args.contains(&"--test".to_string());

    let run_part1 = part1 || (!part1 && !part2);
    let run_part2 = part2 || (!part1 && !part2);
    
    // Fetch input
    // if fetch, force fetching and overwrite existing input file, else just fetch if the file doesn't exist
    fetch_input(path, year, day, fetch);
    if fetch { return }; // exit if fetch is true
    
    // Use test input if test flag is provided
    let path = if test { test_path } else { path };

    // Start timer for reading input
    let now = Instant::now();

    // Read input
    let input: Vec<String> = read_input(path);

    // Stop timer for reading input
    let elapsed = now.elapsed();
    println!("\x1b[33m\x1b[1mTime taken to read input: {}.{:06} seconds\x1b[0m", elapsed.as_secs(), elapsed.subsec_micros());

    if run_part1 {
        // Start timer for part 1
        let now = Instant::now();
        let result = part1::part1(&input);
        // Stop timer for part 1
        let elapsed = now.elapsed();
        println!("\x1b[34m\x1b[1mPart 1 result: {}\x1b[0m", result);
        println!("\x1b[33m\x1b[1mTime taken for part 1: {}.{:06} seconds\x1b[0m", elapsed.as_secs(), elapsed.subsec_micros());
    }

    if run_part2 {
        // Start timer for part 2
        let now = Instant::now();
        let result = part2::part2(&input);
        // Stop timer for part 2
        let elapsed = now.elapsed();
        println!("\x1b[34m\x1b[1mPart 2 result: {}\x1b[0m", result);
        println!("\x1b[33m\x1b[1mTime taken for part 2: {}.{:06} seconds\x1b[0m", elapsed.as_secs(), elapsed.subsec_micros());
    }
}

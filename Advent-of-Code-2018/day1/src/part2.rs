use std::collections::HashSet;

fn parse_input(input: &Vec<String>) -> Vec<i64> {
    let mut result = Vec::new();
    for line in input {
        if let Ok(num) = line.parse::<i64>() {
            result.push(num);
        }
    }
    return result;
}

fn find_first_repeated_frequency(input: &Vec<i64>) -> i64 {
    let mut frequency = 0;
    let mut seen_frequencies = HashSet::new();
    seen_frequencies.insert(frequency);

    loop {
        for num in input {
            frequency += num;
            if seen_frequencies.contains(&frequency) {
                return frequency;
            }
            seen_frequencies.insert(frequency);
        }
    }
}

pub fn part2(input: &Vec<String>) -> i64 {
    let parsed_input = parse_input(input);
    let result = find_first_repeated_frequency(&parsed_input);
    return result;
}
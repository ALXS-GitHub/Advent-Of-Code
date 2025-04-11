fn parse_input(input: &Vec<String>) -> Vec<i64> {
    let mut result = Vec::new();
    for line in input {
        if let Ok(num) = line.parse::<i64>() {
            result.push(num);
        }
    }
    return result;
}

pub fn part1(input: &Vec<String>) -> i64 {
    let parsed_input = parse_input(input);
    let mut frequency = 0;
    for num in parsed_input {
        frequency += num;
    }
    return frequency;
}
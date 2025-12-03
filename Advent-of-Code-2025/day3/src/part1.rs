fn parse_input(input: &Vec<String>) -> Vec<Vec<i64>> {
    let mut parsed: Vec<Vec<i64>> = Vec::new();
    for line in input {
        let row: Vec<i64> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .collect();
        parsed.push(row);
    }
    return parsed;
}

fn find_max_in_range(arr: &Vec<i64>, start: usize, end: usize) -> (i64, usize) {
    let mut max_value = arr[start];
    let mut max_index = start;
    for i in start..end {
        if arr[i] > max_value {
            max_value = arr[i];
            max_index = i;
        }
    }
    return (max_value, max_index);
}

pub fn part1(input: &Vec<String>) -> i64 {
    let banks = parse_input(input);
    let mut total = 0;

    for bank in banks {
        let len = bank.len();
        let (max_battery, max_index) = find_max_in_range(&bank, 0, len - 1);
        let (second_max_battery, _) = find_max_in_range(&bank, max_index + 1, len);
        let composed_battery: i64 = format!("{}{}", max_battery, second_max_battery).parse().unwrap();
        total += composed_battery;
    }

    return total;
}
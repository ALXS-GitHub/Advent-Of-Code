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

pub fn part2(input: &Vec<String>) -> i64 {
    let banks = parse_input(input);
    let mut total = 0;
    let composed_size = 12;

    for bank in banks {
        let len = bank.len();
        let mut composed_battery_str = String::new();
        let mut max_index = 0;
        for i in 0..composed_size {
            if i != 0 { max_index += 1}
            let (current_max_battery, current_max_index) = find_max_in_range(&bank, max_index, len - (composed_size - i - 1));
            composed_battery_str.push_str(&current_max_battery.to_string());
            max_index = current_max_index;
        }
        let composed_battery: i64 = composed_battery_str.parse().unwrap();
        total += composed_battery;
    }

    return total;
}
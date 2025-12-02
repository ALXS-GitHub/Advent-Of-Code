pub struct Range {
    pub start: i64,
    pub end: i64,
}

pub fn parse_input(input: &Vec<String>) -> Vec<Range> {
    let mut ranges: Vec<Range> = Vec::new();

    let ranges_line = &input[0];
    let range_parts: Vec<&str> = ranges_line.split(',').collect(); 
    for range_part in range_parts {
        let parts: Vec<&str> = range_part.split('-').collect();
        let start = parts[0].parse::<i64>().unwrap();
        let end = parts[1].parse::<i64>().unwrap();
        ranges.push(Range { start, end });
    }

    return ranges;
}

pub fn is_invalid_id(id: &String) -> i64 {
    let id_len = id.len();
    for part_size in 1..=(id_len / 2) {
        if id_len % part_size == 0 {
            let num_parts = id_len / part_size;
            let first_part = &id[0..part_size];
            let mut all_parts_equal = true;
            for part_index in 1..num_parts {
                let start_index = part_index * part_size;
                let end_index = start_index + part_size;
                let current_part = &id[start_index..end_index];
                if current_part != first_part {
                    all_parts_equal = false;
                    break;
                }
            }
            if all_parts_equal {
                return id.parse::<i64>().unwrap();
            }
        }
    }
    return 0;
}

pub fn part2(input: &Vec<String>) -> i64 {

    let ranges = parse_input(input);
    let mut invalid_ids_sum: i64 = 0;

    for range in ranges {

        for id_num in range.start..=range.end {
            let id_str = id_num.to_string();
            invalid_ids_sum += is_invalid_id(&id_str);
        }
    }

    return invalid_ids_sum;
}
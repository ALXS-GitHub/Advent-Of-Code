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
    if id_len % 2 != 0 {
        return 0;
    } else {
        let half_len = id_len / 2;
        let first_half = &id[0..half_len];
        let second_half = &id[half_len..id_len];
        if first_half == second_half {
            return id.parse::<i64>().unwrap();
        } else {
            return 0;
        }
    }
}

pub fn part1(input: &Vec<String>) -> i64 {

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
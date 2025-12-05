#[derive(Clone, Debug)]
pub struct Range {
    pub start: i64,
    pub end: i64,
}

pub fn parse_input(input: &Vec<String>) -> (Vec<Range>, Vec<i64>) {
    let mut ranges: Vec<Range> = Vec::new();
    let mut ranges_part = true;
    let mut ingredients: Vec<i64> = Vec::new();

    for line in input {
        if line.is_empty() {
            ranges_part = false;
            continue;
        }
        if ranges_part {
            let parts: Vec<&str> = line.split('-').collect();
            let start: i64 = parts[0].parse().unwrap();
            let end: i64 = parts[1].parse().unwrap();
            ranges.push(Range { start, end });
        } else {
            let ingredient: i64 = line.parse().unwrap();
            ingredients.push(ingredient);
        }
    }

    return (ranges, ingredients);
}

pub fn merge_ranges(ranges: &Vec<Range>) -> Vec<Range> {
    if ranges.is_empty() {
        return Vec::new();
    }

    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort_by_key(|r| r.start);

    let mut merged: Vec<Range> = Vec::new();
    let mut current_range = sorted_ranges[0].clone();

    for range in sorted_ranges.iter().skip(1) {
        if range.start <= current_range.end {
            current_range.end = current_range.end.max(range.end);
        } else {
            merged.push(current_range);
            current_range = range.clone();
        }
    }
    merged.push(current_range);

    return merged;
}

pub fn part2(input: &Vec<String>) -> i64 {
    let (ranges, _) = parse_input(input);
    let merged_ranges = merge_ranges(&ranges);

    let mut total_ranges_length = 0;
    for range in &merged_ranges {
        total_ranges_length += range.end - range.start + 1;
    }

    return total_ranges_length;
}
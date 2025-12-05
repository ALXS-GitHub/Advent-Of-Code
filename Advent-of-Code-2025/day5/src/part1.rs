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

pub fn part1(input: &Vec<String>) -> i64 {
    let (ranges, ingredients) = parse_input(input);
    let mut fresh_count = 0;
    for ingredient in ingredients {
        let mut is_fresh = false;
        for range in &ranges {
            if ingredient >= range.start && ingredient <= range.end {
                is_fresh = true;
                break;
            }
        }
        if is_fresh {
            fresh_count += 1;
        }
    }
    return fresh_count;
}
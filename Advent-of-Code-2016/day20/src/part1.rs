use std::cmp::{min, max};

fn get_max_ranges(ranges: Vec<(u32, u32)>) -> Vec<(u32, u32)> {

    let mut new_ranges = Vec::new();

    for r in ranges.iter() {

        let mut inserted = false;

        for i in 0..new_ranges.len() {
            let nr: (u32, u32) = new_ranges[i];
            if r.0 >= nr.0 && r.0 <= nr.1 {
                new_ranges[i] = (nr.0, nr.1.max(r.1));
                inserted = true;
                break
            }
            if r.1 <= nr.1 && r.1 >= nr.0 {
                new_ranges[i] = (nr.0.min(r.0), nr.1);
                inserted = true;
                break
            }
            if r.0 <= nr.0 && r.1 >= nr.1 {
                new_ranges[i] = (r.0, r.1);
                inserted = true;
                break
            }
        }

        if !inserted {
            new_ranges.push(*r);
        }
    }

    new_ranges

}

fn attach_sorted_ranges(ranges: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    // assume sorted

    let mut new_ranges = Vec::new();
    let mut current_range = ranges[0];

    for i in 1..ranges.len() {
        if current_range.1 >= ranges[i].0 - 1 && current_range.1 <= ranges[i].1 {
            current_range = (current_range.0, ranges[i].1);

        } else {
            new_ranges.push(current_range);
            current_range = ranges[i];
        }
    }

    new_ranges.push(current_range);

    new_ranges
}

pub fn part1(input: &Vec<String>) -> i64 {

    let ranges = input.iter().map(|l| {
        let l = l.split_once("-").unwrap();
        let (min, max) = (l.0.parse::<u32>().unwrap(), l.1.parse::<u32>().unwrap());
        (min, max)
    }).collect::<Vec<_>>();

    let mut new_ranges = get_max_ranges(ranges);
    new_ranges.sort();

    new_ranges = attach_sorted_ranges(new_ranges);

    let lowest_valid_ip = new_ranges[0].1 + 1;

    return lowest_valid_ip as i64;
}